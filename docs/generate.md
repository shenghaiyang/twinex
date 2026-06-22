# Generate

Generate localization files from a Twine file.

---

## `generate`

```sh
twinex generate <TWINE_FILE> <OUTPUT_PATH> [OPTIONS]
```

The output mode is determined by the flags:

- **Single file** (default): Generates one localization file for a specific language.
- **`--all`**: Generates files for all languages into a directory.
- **`--archive`**: Generates a zip archive containing all localization files.

---

## Single file (default)

Generate a single localization file for a specific language and format.

```sh
twinex generate <TWINE_FILE> <OUTPUT_PATH> [OPTIONS]
```

**Examples:**

```sh
# Android XML for Chinese
twinex generate twine.txt strings.xml -f android -l zh

# iOS strings for Chinese with specific encoding
twinex generate twine.txt zh.strings -l zh -e UTF-16

# Filter by tags
twinex generate twine.txt output.xml -f android -t ios,greeting

# Only translated strings
twinex generate twine.txt zh.strings -l zh -i translated
```

---

## All languages (`--all`)

Generate localization files for all languages in a Twine file.

```sh
twinex generate <TWINE_FILE> <OUTPUT_DIR> -a [OPTIONS]
```

**Example:**

```sh
twinex generate twine.txt res/ -f android -r -a
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

## Zip archive (`--archive`)

Generate a zip archive containing all localization files.

```sh
twinex generate <TWINE_FILE> <OUTPUT_ZIP> --archive -f <FORMAT> -l <LANGS>
```

**Example:**

```sh
twinex generate twine.txt translations.zip -f apple -l zh --archive
```

---

## Options

| Option | Description |
|--------|-------------|
| `-f, --format` | Output format (see [Supported Formats](formats.md)) |
| `-l, --lang` | Comma-separated languages to generate (default: all) |
| `-a, --all` | Generate all localization files (output_path is a directory) |
| `--archive` | Generate a zip archive of localization files |
| `-d, --developer-language` | Override the developer language |
| `-t, --tags` | Comma-separated tags to filter (e.g. `ios,greeting`) |
| `-u, --untagged` | Include strings without tags |
| `-i, --include` | Filter: `all` (default), `translated`, `untranslated` |
| `-e, --encoding` | Output encoding (`UTF-8`, `UTF-16`, `UTF-16LE`, `UTF-16BE`) |
| `--escape-all-tags` | Escape all HTML/XML tags in output |
| `--validate` | Validate the Twine file before generating |
| `-r, --create-folders` | (with `--all`) Create language-specific directories |
| `-n, --file-name` | (with `--all`) Custom output file name |