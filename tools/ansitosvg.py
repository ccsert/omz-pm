#!/usr/bin/env python3
"""把 `tmux capture-pane -e -p` 的 ANSI 文本渲染成 SVG 截图。
用法: tmux capture-pane -t <sess> -e -p | python3 tools/ansitosvg.py out.svg [title]"""
import html
import sys
import unicodedata

# xterm 256 色前 16 项(GitHub Dark 风格)
PAL16 = ["#1f2328", "#f0883e", "#3fb950", "#d29922", "#4493f8", "#ab7df8", "#39c5cf", "#d0d7de",
         "#6e7681", "#ffa657", "#7ee787", "#e3b341", "#79c0ff", "#d2a8ff", "#76e3ea", "#ffffff"]
# 256 色中常用的几个补充
EXTRA = {244: "#8b949e", 250: "#c9d1d9", 241: "#6e7681"}
DEFAULT_FG = "#c9d1d9"
DEFAULT_BG = "#161b22"

CHAR_W = 7.4      # ASCII 单元格宽(px)
CJK_W = 14.8      # CJK 单元格宽(px)
LINE_H = 19.0
PAD = 18.0
FS = 13.5


def cell_width(ch: str) -> float:
    return CJK_W if unicodedata.east_asian_width(ch) in ("W", "F") else CHAR_W


def apply_sgr(parts, state):
    """解析一条 SGR 序列的参数,更新 (fg, bg, bold) 状态。"""
    i = 0
    while i < len(parts):
        p = parts[i] or "0"
        n = int(p)
        if n == 0:
            state["fg"], state["bg"], state["bold"] = DEFAULT_FG, None, False
        elif n == 1:
            state["bold"] = True
        elif 30 <= n <= 37:
            state["fg"] = PAL16[n - 30]
        elif 90 <= n <= 97:
            state["fg"] = PAL16[n - 90 + 8]
        elif n == 39:
            state["fg"] = DEFAULT_FG
        elif 40 <= n <= 47:
            state["bg"] = PAL16[n - 40]
        elif 100 <= n <= 107:
            state["bg"] = PAL16[n - 100 + 8]
        elif n == 49:
            state["bg"] = None
        elif n in (38, 48):
            # 38;5;<idx> 或 38;2;r;g;b
            if i + 1 < len(parts) and parts[i + 1] == "5":
                idx = int(parts[i + 2])
                color = PAL16[idx] if idx < 16 else EXTRA.get(idx, f"#{idx:02x}{idx:02x}{idx:02x}")
                if n == 38:
                    state["fg"] = color
                else:
                    state["bg"] = color
                i += 2
            elif i + 1 < len(parts) and parts[i + 1] == "2":
                r, g, b = (int(x) for x in parts[i + 2: i + 5])
                if n == 38:
                    state["fg"] = f"#{r:02x}{g:02x}{b:02x}"
                else:
                    state["bg"] = f"#{r:02x}{g:02x}{b:02x}"
                i += 4
        i += 1


def parse(lines):
    """返回 {row: [(display_col, char, fg, bg, bold), ...]}"""
    grid = {}
    max_disp = 0
    for row, raw in enumerate(lines):
        row += 1  # 第 0 行留白
        cells = []
        col = 0.0
        state = {"fg": DEFAULT_FG, "bg": None, "bold": False}
        i = 0
        text = raw.rstrip("\n")
        while i < len(text):
            c = text[i]
            if c == "\x1b" and i + 1 < len(text) and text[i + 1] == "[":
                j = text.index("m", i)
                apply_sgr(text[i + 2: j].split(";"), state)
                i = j + 1
                continue
            cells.append((col, c, state["fg"], state["bg"], state["bold"]))
            w = 2.0 if unicodedata.east_asian_width(c) in ("W", "F") else 1.0
            col += w
            i += 1
        max_disp = max(max_disp, col)
        grid[row] = cells
    return grid, max_disp


def runs_of(cells):
    runs = []
    for col, ch, fg, bg, bold in cells:
        if runs and runs[-1][1] == (fg, bg, bold):
            runs[-1][2].append(ch)
        else:
            runs.append([col, (fg, bg, bold), [ch]])
    return runs


def main():
    out_path = sys.argv[1]
    lines = sys.stdin.read().split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    grid, max_disp = parse(lines)
    rows = (max(grid) + 1) if grid else 1
    W = PAD * 2 + max(max_disp + 2, 80) * CHAR_W
    H = PAD * 2 + rows * LINE_H

    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W:.0f}" height="{H:.0f}" '
        f'viewBox="0 0 {W:.0f} {H:.0f}">',
        f'<rect width="{W:.0f}" height="{H:.0f}" rx="10" fill="{DEFAULT_BG}"/>',
    ]
    for row in sorted(grid):
        y = PAD + row * LINE_H + FS
        for col, (fg, bg, bold), chars in runs_of(grid[row]):
            x = PAD + col * CHAR_W
            text = "".join(chars)
            if bg:
                w = sum(cell_width(c) for c in chars)
                parts.append(
                    f'<rect x="{x:.1f}" y="{y - FS - 3:.1f}" width="{w:.1f}" '
                    f'height="{LINE_H:.1f}" fill="{bg}" rx="2"/>'
                )
            weight = ' font-weight="600"' if bold else ""
            parts.append(
                f'<text x="{x:.1f}" y="{y:.1f}" fill="{fg}"{weight} '
                f'font-family="\'SF Mono\',\'JetBrains Mono\',Menlo,Consolas,monospace" '
                f'font-size="{FS}" xml:space="preserve">{html.escape(text)}</text>'
            )
    parts.append("</svg>")
    with open(out_path, "w") as f:
        f.write("\n".join(parts))
    print(f"{out_path}: {W:.0f}x{H:.0f}, {rows} 行")


if __name__ == "__main__":
    main()
