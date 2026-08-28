#!/usr/bin/env python3
"""把 data/readmes-zh/<插件>.md 整篇中文译文打包为 data/readmes_zh.json,
供 src/readme.rs 编译期 include_str! 嵌入。

校验(任一失败即报错退出,不产出文件):
1. 覆盖率:data/translations.json 里的每个插件都必须有译文,且无多余文件;
2. 结构:非空、含中文、含 ✅ 启用行、代码围栏闭合、无遗留英文样板启用段;
3. 表格:译文表格行数不少于源 README(数据行原样保留,只译表头/说明列);
   标题数量与源 README 一致。
源 README 从 $OMZ_ZSH(默认 ~/.oh-my-zsh)读取,缺失时跳过对比项。"""

import json
import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ZH_DIR = ROOT / "data" / "readmes-zh"
DICT = ROOT / "data" / "translations.json"
OUT = ROOT / "data" / "readmes_zh.json"
OMZ = Path(os.environ.get("OMZ_ZSH", Path.home() / ".oh-my-zsh"))

CJK = re.compile(r"[\u4e00-\u9fff]")


def headings(text: str) -> int:
    """只数行首(无缩进)的 ATX 标题,跳过围栏代码块——
    缩进的 `#` 多为代码注释,不是标题。"""
    n, in_fence = 0, False
    for l in text.splitlines():
        if l.lstrip().startswith("```"):
            in_fence = not in_fence
            continue
        if not in_fence and re.match(r"#{1,6} ", l):
            n += 1
    return n


def table_lines(text: str) -> int:
    return sum(1 for l in text.splitlines() if l.lstrip().startswith("|"))


def fence_lines(text: str) -> int:
    return sum(1 for l in text.splitlines() if l.strip().startswith("```"))


def validate(name: str, text: str, src: str | None) -> list[str]:
    errs = []
    if not text.strip():
        return [f"{name}: 译文为空"]
    if not CJK.search(text):
        errs.append(f"{name}: 无中文内容")
    # 标准样板启用段的可靠特征是「plugins array」字样;
    # 仅 "to use it"/"to enable it" 可能只是正文措辞,不算。
    has_boilerplate = bool(src and "plugins array" in src.lower())
    if has_boilerplate and "✅ 启用方式" not in text:
        errs.append(f"{name}: 源文有启用样板段,译文缺 ✅ 启用方式行")
    if "plugins array" in text or re.search(r"To use it, add", text):
        errs.append(f"{name}: 遗留英文样板启用段")
    if fence_lines(text) % 2:
        errs.append(f"{name}: 代码围栏不闭合")
    if src:
        if headings(text) != headings(src):
            errs.append(f"{name}: 标题数 {headings(text)} != 源 {headings(src)}")
        if table_lines(text) < table_lines(src):
            errs.append(
                f"{name}: 表格行 {table_lines(text)} < 源 {table_lines(src)}(数据行必须保留)"
            )
    return errs


def main() -> int:
    allow_partial = "--allow-partial" in sys.argv
    expected = set(json.loads(DICT.read_text(encoding="utf-8")))
    actual = {p.stem for p in ZH_DIR.glob("*.md")}
    problems = []
    missing = expected - actual
    extra = actual - expected
    if missing:
        msg = f"缺少译文的插件({len(missing)}): {sorted(missing)}"
        if allow_partial:
            print(f"⚠ {msg}")
        else:
            problems.append(msg)
    if extra:
        problems.append(f"多余译文文件({len(extra)}): {sorted(extra)}")

    bundle = {}
    for name in sorted(actual & expected):
        text = (ZH_DIR / f"{name}.md").read_text(encoding="utf-8")
        src_path = OMZ / "plugins" / name / "README.md"
        src = src_path.read_text(encoding="utf-8") if src_path.is_file() else None
        bundle[name] = text
        problems.extend(validate(name, text, src))

    if problems:
        for p in problems:
            print(f"✗ {p}")
        return 1

    OUT.write_text(
        json.dumps(bundle, ensure_ascii=False, sort_keys=True, separators=(",", ":")),
        encoding="utf-8",
    )
    print(f"✓ 打包 {len(bundle)} 篇译文 → {OUT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
