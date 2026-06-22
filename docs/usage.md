# Usage Overview

## Command Overview

```
twinex <COMMAND>

Commands:
  generate  Generate localization file(s) from a twine file
  consume   Consume translations from localization file(s) into the twine file
  validate  Validate that a twine file is parseable
```

## Quick Reference

### Generate

[:octicons-arrow-right-24: Full generate reference](generate.md)

| Mode | Flag | Description |
|------|------|-------------|
| Single file | *(default)* | Generate a single file for one language |
| All languages | `--all` | Generate files for all languages |
| Zip archive | `--archive` | Generate a zip archive |

```sh
# Single file
twinex generate twine.txt output.xml -f android -l zh

# All languages
twinex generate twine.txt res/ -f android -r -a

# Zip archive
twinex generate twine.txt translations.zip -f apple -l zh --archive
```

### Consume

[:octicons-arrow-right-24: Full consume reference](consume.md)

| Mode | Flag | Description |
|------|------|-------------|
| Single file | *(default)* | Consume a single file |
| All files | `--all` | Consume files from a directory |
| Zip archive | *(auto)* | Consume files from a zip archive (detected by `.zip` extension) |

```sh
# Single file
twinex consume twine.txt zh.strings -l zh

# All files in directory
twinex consume twine.txt Resources/ -f apple -a
```

### Validate

[:octicons-arrow-right-24: Full validate reference](validate.md)

```sh
twinex validate twine.txt
twinex validate twine.txt -p    # pedantic mode
```

### Common Concepts

| Page | Description |
|------|-------------|
| [Tag Filtering](tag-filtering.md) | Control which strings are included via tags |
| [Encoding](encoding.md) | UTF-8 / UTF-16 output encoding and fallback behavior |
| [Supported Formats](formats.md) | Full reference of all 7 output formats |