---
hide:
  - navigation
---

# Twinex

A localization code generator compatible with the [Twine](https://github.com/scelis/twine) file format, implemented in Rust.

[![Crates.io](https://img.shields.io/crates/v/twinex-cli)](https://crates.io/crates/twinex-cli)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](https://github.com/shenghaiyang/twinex/blob/main/LICENSE)

---

## What is Twinex?

Twinex reads a [Twine-formatted](https://github.com/scelis/twine) text file containing your app's translations, and generates platform-specific localization files. It also works in reverse — consuming existing localization files back into a Twine file.

=== "Generate"

    ```sh
    twinex generate-localization-file twine.txt output/strings.xml -f android -l zh
    ```

=== "Consume"

    ```sh
    twinex consume-localization-file twine.txt zh.strings -l zh
    ```

=== "Validate"

    ```sh
    twinex validate-twine-file twine.txt --pedantic
    ```

## Supported Formats

| Format | Extension | Platform |
|--------|-----------|----------|
| `apple` | `.strings` | iOS / macOS |
| `android` | `.xml` | Android |
| `arb` | `.arb` | Flutter / Dart |
| `django` | `.po` | Django |
| `flash` | `.properties` | Flash |
| `gettext` | `.po` | GNU Gettext |
| `jquery` | `.json` | jQuery i18n |

## License

Apache 2.0 — [View License](https://github.com/shenghaiyang/twinex/blob/main/LICENSE)
