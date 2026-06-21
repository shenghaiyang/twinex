# Generate

Generate localization files from a Twine file.

---

## `generate-localization-file`

Generate a single localization file for a specific language and format.

```sh
twinex generate-localization-file <TWINE_FILE> <OUTPUT_PATH> [OPTIONS]
```

**Examples:**

```sh
# Android XML for Chinese
twinex generate-localization-file twine.txt strings.xml -f android -l zh

# iOS strings for Chinese with specific encoding
twinex generate-localization-file twine.txt zh.strings -l zh -e UTF-16

# Filter by tags
twinex generate-localization-file twine.txt output.xml -f android -t ios,greeting

# Only translated strings
twinex generate-localization-file twine.txt zh.strings -l zh -i translated
```

---

## `generate-all-localization-files`

Generate localization files for all languages in a Twine file.

```sh
twinex generate-all-localization-files <TWINE_FILE> <OUTPUT_DIR> [OPTIONS]
```

**Example:**

```sh
twinex generate-all-localization-files twine.txt res/ -f android -r
```

This creates:

```
res/
├── values/
│   └── strings.xml
├── values-zh/
│   └── strings.xml
└── values-ja/
    └── strings.xml
```

---

## `generate-localization-archive`

Generate a zip archive containing all localization files.

```sh
twinex generate-localization-archive <TWINE_FILE> <OUTPUT_ZIP> -f <FORMAT> -l <LANGS>
```

**Example:**

```sh
twinex generate-localization-archive twine.txt translations.zip -f apple -l zh
```

---

## Options

| Option | Description |
|--------|-------------|
| `-f, --format` | Output format (see [Supported Formats](formats.md)) |
| `-l, --lang` | Comma-separated languages to generate (default: all) |
| `-d, --developer-language` | Override the developer language |
| `-t, --tags` | Comma-separated tags to filter (e.g. `ios,greeting`) |
| `-u, --untagged` | Include strings without tags |
| `-i, --include` | Filter: `all` (default), `translated`, `untranslated` |
| `-e, --encoding` | Output encoding (`UTF-8`, `UTF-16`, `UTF-16LE`, `UTF-16BE`) |
| `--escape-all-tags` | Escape all HTML/XML tags in output |
| `--validate` | Validate the Twine file before generating |
| `-r, --create-folders` | Create language-specific directories (`generate-all` only) |
| `-n, --file-name` | Custom output file name (`generate-all` only) |