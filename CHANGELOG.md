# Changelog

All notable changes to this project are documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

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
