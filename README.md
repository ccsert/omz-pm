<div align="center">

# omz-pm

**A TUI plugin & theme manager for Oh My Zsh — with a built-in Chinese dictionary.**

Browse all 360+ built-in plugins, read what they *actually do* (in Chinese or English),
toggle them with a single keypress, preview & try-on themes live — all without ever
hand-editing `~/.zshrc`.

[![CI](https://github.com/ccsert/omz-pm/actions/workflows/ci.yml/badge.svg)](https://github.com/ccsert/omz-pm/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/ccsert/omz-pm)](https://github.com/ccsert/omz-pm/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.81%2B-orange.svg)](https://www.rust-lang.org)

[English](README.md) | [简体中文](README.zh-CN.md)

<img src="docs/screenshots/plugins.svg" alt="omz-pm plugin view" width="100%">

</div>

## Why

Oh My Zsh ships **359 plugins and 144 themes**, but discovering them means scrolling
through a giant English wiki page, guessing what `zsh-navigation-tools` does, and
hand-editing `plugins=(...)` in your zshrc.

omz-pm puts all of that in a terminal UI:

- **See everything** — plugins *and* themes, enabled state at a glance
- **Understand everything** — every plugin ships with a curated Chinese summary,
  practical usage guide, and annotated aliases; READMEs are one keypress away
- **Change safely** — diff preview before every write, timestamped backups, one-command rollback

## Features

### 🔌 Plugin management

| | |
| --- | --- |
| **Browse & search** | All built-in + custom plugins, with live search (names, Chinese descriptions, categories) and 18 category filters |
| **Toggle & save** | `Space` to stage enable/disable changes, `s` to preview a diff, `Enter` to write — atomic, always backed up |
| **Chinese dictionary** | 359 curated entries baked into the binary: summary, usage guide, annotated aliases for the 48 most popular plugins |
| **Alias index** | Aliases are extracted from each plugin's source code — works for custom plugins too |
| **README reader** | Read any plugin's README in-TUI: section headers localized, boilerplate translated, alias tables kept as-is |

### 🎨 Theme management (`T`)

| | |
| --- | --- |
| **Browse** | All 144 built-in themes + `$ZSH_CUSTOM/themes`, current theme highlighted |
| **Preview** | Right panel renders the theme's real prompt via `print -P`; `p` flashes it in true color |
| **Try-on** | `i` spawns a full interactive zsh with the theme applied — exit to come back |
| **Enable** | `Enter` rewrites `ZSH_THEME=` through the same diff-confirm-backup pipeline |

### 🛟 Safety net

- Every write: **diff preview → confirm → timestamped backup → temp file + atomic rename**
- `b` opens the backup browser; restoring snapshots your current zshrc first, so rollback is always possible
- Parser handles single/multi-line arrays, `plugins+=`, inline comments, quoting, `$var` entries —
  everything else in your zshrc is preserved byte-for-byte

### ⏱️ Benchmarks

`omz-pm bench` times each enabled plugin in an isolated zsh (median of N runs, warm-up excluded)
so you can see exactly what's slowing your startup down.

## Screenshots

| Themes (live preview) | Save with diff preview |
| --- | --- |
| <img src="docs/screenshots/themes.svg" width="100%"> | <img src="docs/screenshots/diff.svg" width="100%"> |

| README reader (localized) | — |
| --- | --- |
| <img src="docs/screenshots/readme.svg" width="100%"> | |

## Install

**From source** (needs Rust 1.81+):

```bash
cargo install --git https://github.com/ccsert/omz-pm
# or clone + ./install.sh to also symlink into ~/.local/bin
```

**Prebuilt binaries**: grab one from [Releases](https://github.com/ccsert/omz-pm/releases)
(`aarch64/x86_64` × `macOS/Linux`), untar, put `omz-pm` on your `PATH`.

Requirements: `zsh` + Oh My Zsh. That's it — the dictionary is compiled in, nothing is fetched at runtime.

## Usage

```bash
omz-pm            # TUI (default)
omz-pm bench      # which plugins slow down your startup?
```

### Key bindings

| Key | Action |
| --- | --- |
| `↑↓` / `j k` | Move |
| `Space` / `Enter` | Toggle enable ↔ disable |
| `Tab` / `Shift+Tab` | Filter: all → enabled → disabled |
| `c` / `C` | Cycle 18 category filters |
| `/` | Search (names, Chinese text, categories) |
| `r` | Read plugin README (localized) |
| `s` | Save — opens diff preview first |
| `b` | Backup browser & restore |
| `T` | Switch plugin ↔ theme view |
| `i` / `p` | (themes) try-on in a live zsh / flash colored preview |
| `?` / `q` | Help / quit |

### CLI

```bash
omz-pm list [--enabled|--disabled]  # plugin inventory
omz-pm info <name>                  # description + usage guide + annotated aliases
omz-pm which <alias>                # gco ← git plugin: git checkout
omz-pm aliases <name>               # every alias a plugin defines
omz-pm readme <name>                # print the localized README
omz-pm themes                       # list themes
omz-pm theme <name>                 # enable a theme (diff-confirmed)
omz-pm theme --preview <name>       # render its prompt in color
omz-pm bench [--runs N] [--all]     # load-time analysis
omz-pm backups [--clean --keep N]   # backup management
omz-pm restore <index|path>         # roll back
omz-pm enable/disable <name>...     # scriptable enable/disable
```

`--zshrc <path>` works everywhere for testing or non-default setups.

## Customizing translations

The Chinese dictionary can be overridden without recompiling — drop entries into
`~/.config/omz-pm/translations.json` (fields: `summary`, `detail`, `cat`, `usage`, `aliases`).
This is also the place to document your own custom plugins:

```json
{
  "my-plugin": {
    "summary": "Does one thing well",
    "usage": "Press Ctrl+K to ...",
    "aliases": {"mp": "what it means"}
  }
}
```

`tools/enrich_translations.py` regenerates the baked-in dictionary (categories + usage corpus).

## How it works

```
$ZSH/plugins/*          ─┐
$ZSH_CUSTOM/plugins/*   ─┤→ scanner → ┌ TUI (ratatui) ─┐
~/.zshrc                ─┤            │ CLI subcommands│
data/translations.json  ─┘ (baked in) └────────────────┘
                                   │
                        diff → backup → atomic write
```

## Development

```bash
cargo test      # 50 unit tests (zshrc round-trips, alias parser, diff, dictionary, layout)
cargo clippy    # zero warnings
```

Contributions welcome — especially usage-guide entries for the remaining ~300 plugins
(edit `tools/enrich_translations.py`, run it, and `data/translations.json` is regenerated).

## License

[MIT](LICENSE) © ccsert
