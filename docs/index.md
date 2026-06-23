---
hide:
  - navigation
---

# Twinex

A localization code generator compatible with the [Twine](https://github.com/scelis/twine) file format, implemented in Rust.

[![Crates.io][crates-badge]][crates-url]
[![License][license-badge]][license-url]
[![Docs][docs-action-badge]][docs-action-url]
[![CI][ci-action-badge]][ci-action-url]
[![Website][website-badge]][website-url]

[crates-badge]: https://img.shields.io/crates/v/twinex-cli.svg
[crates-url]: https://crates.io/crates/twinex-cli
[license-badge]: https://img.shields.io/badge/license-Apache%202.0-blue.svg
[license-url]: https://github.com/shenghaiyang/twinex/blob/trunk/LICENSE
[docs-action-badge]: https://github.com/shenghaiyang/twinex/actions/workflows/docs.yml/badge.svg
[docs-action-url]: https://github.com/shenghaiyang/twinex/actions/workflows/docs.yml
[ci-action-badge]: https://github.com/shenghaiyang/twinex/actions/workflows/ci.yml/badge.svg
[ci-action-url]: https://github.com/shenghaiyang/twinex/actions/workflows/ci.yml
[website-badge]: https://img.shields.io/badge/website-twinex-blue
[website-url]: https://shenghaiyang.github.io/twinex/

---

## What is Twinex?

Twinex reads a [Twine-formatted](https://github.com/scelis/twine) text file containing your app's translations, and generates platform-specific localization files. It also works in reverse — consuming existing localization files back into a Twine file.

=== "Generate"

    ```sh
    twinex generate twine.txt output/strings.xml -f android -l zh
    ```

=== "Consume"

    ```sh
    twinex consume twine.txt zh.strings -l zh
    ```

=== "Validate"

    ```sh
    twinex validate twine.txt --pedantic
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

## Status

!!! warning

    Twinex is under active development. Until version 1.0, breaking changes may occur in any release.


## License

    Copyright 2026 shenghaiyang
    
    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at
    
           http://www.apache.org/licenses/LICENSE-2.0
    
    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
