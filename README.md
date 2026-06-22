# Twinex

A localization code generator compatible with the [Twine](https://github.com/scelis/twine) file format, implemented in Rust.

[![Crates.io][crates-badge]][crates-url]
[![License][license-badge]][license-url]
[![Docs][docs-action-badge]][docs-action-url]
[![CI][ci-action-badge]][ci-action-url]

[crates-badge]: https://img.shields.io/crates/v/twinex-cli.svg
[crates-url]: https://crates.io/crates/twinex-cli
[license-badge]: https://img.shields.io/badge/license-Apache%202.0-blue.svg
[license-url]: https://github.com/shenghaiyang/twinex/blob/trunk/LICENSE
[docs-action-badge]: https://github.com/shenghaiyang/twinex/actions/workflows/docs.yml/badge.svg
[docs-action-url]: https://github.com/shenghaiyang/twinex/actions/workflows/docs.yml
[ci-action-badge]: https://github.com/shenghaiyang/twinex/actions/workflows/ci.yml/badge.svg
[ci-action-url]: https://github.com/shenghaiyang/twinex/actions/workflows/ci.yml

---

## Status

> [!IMPORTANT]  
> Twinex is under active development. Until version 1.0, breaking changes may occur in any release.

## Usage

### Installation

```sh
cargo install twinex-cli
```

### Twine file format

A Twine file is a plain text file containing sections and key-value definitions:

```
[[Section]]
	[key]
		comment = Description of the key
		tags = ios,android
		en = English translation
		fr = French translation
	[another_key]
		ref = key
		en = Translation
```

### Subcommands

```
twinex <COMMAND>

Commands:
  generate  Generate localization file(s) from a twine file
  consume   Consume translations from localization file(s) into the twine file
  validate  Validate that a twine file is parseable
```

### Generate

Generate localization files from a Twine file.

```sh
# Single file
twinex generate twine.txt output/strings.xml -f android -l ja,ko

# All languages (--all), one file per language
twinex generate twine.txt output/ -f apple -r -a

# Zip archive (--archive)
twinex generate twine.txt output.zip -f apple -l en,fr,ja --archive
```

**Generate options:**

| Option | Description |
|--------|-------------|
| `-f, --format` | Output format: `apple`, `android`, `arb`, `django`, `flash`, `gettext`, `jquery` |
| `-l, --lang` | Comma-separated list of languages to generate (defaults to all) |
| `-a, --all` | Generate all localization files (output_path is a directory) |
| `--archive` | Generate a zip archive of localization files |
| `-d, --developer-language` | Override the developer language |
| `-t, --tags` | Comma-separated list of tags to include in output |
| `-u, --untagged` | Include untagged strings in output |
| `-i, --include` | Filter: `all` (default), `translated`, `untranslated` |
| `-e, --encoding` | Output file encoding (e.g. `UTF-16`, `UTF-16LE`) |
| `--escape-all-tags` | Escape all HTML/XML tags in generated output |
| `--validate` | Validate the Twine file before generating |
| `-r, --create-folders` | (with `--all`) Create language-specific output directories |
| `-n, --file-name` | (with `--all`) Custom output file name |

### Consume

Consume existing localization files back into a Twine file.

```sh
# Single file
twinex consume twine.txt ja.strings -l ja

# All files in a directory (--all)
twinex consume twine.txt Resources/ -f apple -a

# From a zip archive (auto-detected by .zip extension)
twinex consume twine.txt archive.zip
```

**Consume options:**

| Option | Description |
|--------|-------------|
| `-f, --format` | Input format (auto-detected from extension if omitted) |
| `-l, --lang` | Comma-separated list of languages to consume |
| `-a, --all` | Consume all localization files from a directory |
| `-c, --consume-all` | Consume all translations, even if key doesn't exist in twine file |
| `-m, --consume-comments` | Consume comments from the localization file |
| `-d, --developer-language` | Override the developer language |
| `-t, --tags` | Comma-separated list of tags to set on consumed definitions |
| `-o, --output-file` | Write the updated twine data to a different file |
| `-e, --encoding` | Input file encoding |

### Validate

Check that a Twine file is well-formed and optionally enforce naming conventions.

```sh
# Basic validation (parse errors)
twinex validate twine.txt

# Pedantic mode — keys must match ^[A-Za-z0-9_]+$
twinex validate twine.txt -p
```

### Supported formats

| Format | Extension | Description |
|--------|-----------|-------------|
| `apple` | `.strings` | Apple/iOS `.strings` files |
| `android` | `.xml` | Android `res/values/strings.xml` |
| `arb` | `.arb` | Flutter/Dart ARB (Application Resource Bundle) |
| `django` | `.po` | Django `.po` files |
| `flash` | `.properties` | Flash `.properties` files |
| `gettext` | `.po` | GNU Gettext `.po` files |
| `jquery` | `.json` | jQuery i18n `.json` files |

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