# Changelog

All notable changes to this project are documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- **Markdown rendering in the README reader** (`r`): headings, bold/italic,
  inline code, links (label only), `<kbd>` keys, fenced code blocks with a
  gutter, blockquotes, rules and lists are rendered instead of shown raw;
  tables are column-aligned (CJK-aware widths) and wide tables wrap cells
  instead of clipping. Zero new dependencies (`src/markdown.rs`).
- **Complete usage corpus — 359/359 plugins**: every built-in plugin now has a
  practical usage guide plus curated Chinese alias annotations (previously 48).
  Corpus lives in `tools/usage/<issue#>-<category>.json`; aliases are verified
  against plugin sources via `tools/enrich_translations.py --check-sources`.
- **Complete Chinese translations of all 359 built-in plugin READMEs** (`data/readmes-zh/`,
  packed into the compile-time `data/readmes_zh.json`): every section, table and
  note faithfully translated — alias/command tables preserved, boilerplate enable
  paragraphs replaced by a standard Chinese note. The README reader (`r`,
  `omz-pm readme`) now shows the full translation instead of light on-the-fly
  localization; custom plugins keep the previous fallback behavior.
- User overrides: `~/.config/omz-pm/readmes-zh/<plugin>.md` replaces the built-in
  translation without recompiling (also works for custom plugins).
- `tools/build_readme_bundle.py` validates the translations against each source
  README (heading/table/fence parity, coverage of all dictionary entries) and
  regenerates the bundle; style guide in `docs/readme-translation-guide.md`.

## [0.3.0] - 2026-08-27

### Added

- **Theme management** (`T`): browse 144 built-in + custom themes, live prompt
  preview rendered via `print -P`, colored flash preview (`p`), try-on in a real
  interactive zsh (`i`), and diff-confirmed enabling of `ZSH_THEME=`.
- **README reader** (`r`, `omz-pm readme`): read any plugin's README in-TUI with
  localized section headers, translated boilerplate ("To use it, add ... to the
  plugins array"), and translated dependency notes.
- **Backup & restore**: in-TUI backup browser (`b`) and `omz-pm backups` /
  `restore <index|path>`; restoring snapshots the current zshrc first;
  `--clean --keep N` prunes old backups.
- **Load-time benchmarks**: `omz-pm bench [--runs N] [--all]` times each enabled
  plugin in an isolated zsh (warm-up excluded, median reported).
- `omz-pm which <alias>` reverse lookup and `omz-pm aliases <name>` listing.

### Fixed

- Backups created within the same second no longer overwrite each other
  (numeric `-N` suffix).

## [0.2.0] - 2026-08-27

### Added

- **Categories**: 18 Chinese category labels baked into the dictionary, browsable
  via `c` / `C` and searchable.
- **Practical usage guides** for 48 popular plugins: keyboard shortcuts, commands,
  and workflows — plus curated Chinese annotations for common aliases (git, kubectl,
  docker, brew, ...).
- **Alias extraction**: aliases are parsed from each plugin's source code and shown
  in the detail panel, indexed for `which`-style reverse lookup.
- Save flow now shows a **unified diff preview** (LCS-based) before writing.

### Fixed

- Dictionary corrections: copybuffer is bound to `Ctrl+O` (was mis-documented as
  `Ctrl+Z`), `gitignore` sources templates from gitignore.io, and others.

## [0.1.0] - 2026-08-27

### Added

- Initial release.
- TUI for browsing 360+ built-in & custom OMZ plugins with enable/disable toggling.
- Built-in Chinese dictionary covering all 359 official plugins (compiled into the
  binary, no network access).
- Safe zshrc editing: `plugins=(...)` parser (single/multi-line, `plugins+=`,
  comments, quoting, `$var`), diff-confirmed writes, timestamped backups, atomic
  rename, byte-exact preservation of everything else.
- CLI subcommands: `list`, `info`, `enable`, `disable`.
- CJK-aware text layout (wrapping, truncation, padding by display width).
- User translation overrides at `~/.config/omz-pm/translations.json`.
