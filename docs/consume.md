# Consume

Consume existing localization files back into a Twine file.

---

## `consume-localization-file`

Consume a single localization file back into the Twine file.

```sh
twinex consume-localization-file <TWINE_FILE> <INPUT_PATH> [OPTIONS]
```

**Examples:**

```sh
# Consume a Chinese strings file
twinex consume-localization-file twine.txt zh.strings -l zh

# Consume and write to a new file
twinex consume-localization-file twine.txt zh.xml -f android -o twine_updated.txt

# Consume all keys, add tags
twinex consume-localization-file twine.txt input.xml -a -t imported
```

---

## `consume-all-localization-files`

Consume all localization files from a directory.

```sh
twinex consume-all-localization-files <TWINE_FILE> <INPUT_DIR> [OPTIONS]
```

**Example:**

```sh
twinex consume-all-localization-files twine.txt Resources/ -f apple
```

---

## `consume-localization-archive`

Consume localization files from a zip archive.

```sh
twinex consume-localization-archive <TWINE_FILE> <INPUT_ZIP> [OPTIONS]
```

---

## Options

| Option | Description |
|--------|-------------|
| `-f, --format` | Input format (auto-detected from extension if omitted) |
| `-l, --lang` | Comma-separated languages to consume |
| `-a, --consume-all` | Consume all keys, even if they don't exist in the Twine file |
| `-c, --consume-comments` | Consume comments from the localization file |
| `-d, --developer-language` | Override the developer language |
| `-t, --tags` | Tags to set on consumed definitions |
| `-o, --output-file` | Write the updated Twine data to a different file |
| `-e, --encoding` | Input file encoding |