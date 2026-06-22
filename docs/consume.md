# Consume

Consume existing localization files back into a Twine file.

---

## `consume`

```sh
twinex consume <TWINE_FILE> <INPUT_PATH> [OPTIONS]
```

The input mode is determined by the path and flags:

- **Single file** (default): Consume a single localization file.
- **`--all`**: Consume all localization files from a directory.
- **Zip archive** (auto): Consume files from a zip archive (detected by `.zip` extension).

---

## Single file (default)

Consume a single localization file back into the Twine file.

```sh
twinex consume <TWINE_FILE> <INPUT_PATH> [OPTIONS]
```

**Examples:**

```sh
# Consume a Chinese strings file
twinex consume twine.txt zh.strings -l zh

# Consume and write to a new file
twinex consume twine.txt zh.xml -f android -o twine_updated.txt

# Consume all keys, add tags
twinex consume twine.txt input.xml -c -t imported
```

---

## All files (`--all`)

Consume all localization files from a directory.

```sh
twinex consume <TWINE_FILE> <INPUT_DIR> -a [OPTIONS]
```

**Example:**

```sh
twinex consume twine.txt Resources/ -f apple -a
```

---

## Zip archive (auto)

Consume localization files from a zip archive. Detected automatically by the `.zip` extension.

```sh
twinex consume <TWINE_FILE> <INPUT_ZIP> [OPTIONS]
```

---

## Options

| Option | Description |
|--------|-------------|
| `-f, --format` | Input format (auto-detected from extension if omitted) |
| `-l, --lang` | Comma-separated languages to consume |
| `-a, --all` | Consume all localization files from a directory |
| `-c, --consume-all` | Consume all keys, even if they don't exist in the Twine file |
| `-m, --consume-comments` | Consume comments from the localization file |
| `-d, --developer-language` | Override the developer language |
| `-t, --tags` | Tags to set on consumed definitions |
| `-o, --output-file` | Write the updated Twine data to a different file |
| `-e, --encoding` | Input file encoding |