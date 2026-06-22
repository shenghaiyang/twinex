# Change Log

## Unreleased


## 0.2.0

### New

- Documentation website powered by Zensical
- Automated GitHub Pages deployment via GitHub Actions
- **Breaking:** Consolidated 7 subcommands into 3: `generate`, `consume`, `validate`. Use `--all`/`--archive` flags to switch modes.
  - `generate-localization-file` → `generate`
  - `generate-all-localization-files` → `generate --all`
  - `generate-localization-archive` → `generate --archive`
  - `consume-localization-file` → `consume`
  - `consume-all-localization-files` → `consume --all`
  - `consume-localization-archive` → `consume` (auto-detected by `.zip` extension)
  - `validate-twine-file` → `validate`
- **Breaking:** `--consume-all` short flag changed from `-a` to `-c` (now `-a` is `--all` for directory mode)
- Refactored internal architecture: extracted `Twinex` core struct with parameter structs, decoupled from CLI parsing
- Add cargo-dist release workflow

## 0.1.1

### Fixes

- Corrected the installation command in the README from cargo install twinex to cargo install twinex-cli.

## 0.1.0

### New

- Initial release of Twinex
- 7 subcommands: `generate-localization-file`, `generate-all-localization-files`, `generate-localization-archive`, `consume-localization-file`, `consume-all-localization-files`, `consume-localization-archive`, `validate-twine-file`
- 7 output formats: Apple (`.strings`), Android (`.xml`), ARB (`.arb`), Django (`.po`), Flash (`.properties`), Gettext (`.po`), jQuery (`.json`)
- Tag-based string filtering with `-t` and `-u` flags
- Translation fallback to developer language
- Multiple encoding support (`UTF-8`, `UTF-16`, `UTF-16LE`, `UTF-16BE`)
- Pedantic key validation mode (`-p`)
- Extensible formatter registry for third-party format support