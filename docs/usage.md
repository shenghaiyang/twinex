# Usage Overview

## Command Overview

```
twinex <COMMAND>

Commands:
  generate-localization-file       Generate a single localization file
  generate-all-localization-files  Generate all localization files for a project
  generate-localization-archive    Generate a zip archive of localization files
  consume-localization-file        Consume translations from a localization file
  consume-all-localization-files   Consume translations from a directory
  consume-localization-archive     Consume translations from an archive
  validate-twine-file              Validate that a Twine file is parseable
```

## Quick Reference

### Generate

[:octicons-arrow-right-24: Full generate reference](generate.md)

| Command | Description |
|---------|-------------|
| `generate-localization-file` | Generate a single file for one language |
| `generate-all-localization-files` | Generate files for all languages |
| `generate-localization-archive` | Generate a zip archive |

```sh
# Single file
twinex generate-localization-file twine.txt output.xml -f android -l zh

# All languages
twinex generate-all-localization-files twine.txt res/ -f android -r

# Zip archive
twinex generate-localization-archive twine.txt translations.zip -f apple -l zh
```

### Consume

[:octicons-arrow-right-24: Full consume reference](consume.md)

| Command | Description |
|---------|-------------|
| `consume-localization-file` | Consume a single file |
| `consume-all-localization-files` | Consume files from a directory |
| `consume-localization-archive` | Consume files from a zip archive |

```sh
# Single file
twinex consume-localization-file twine.txt zh.strings -l zh

# All files in directory
twinex consume-all-localization-files twine.txt Resources/ -f apple
```

### Validate

[:octicons-arrow-right-24: Full validate reference](validate.md)

```sh
twinex validate-twine-file twine.txt
twinex validate-twine-file twine.txt -p    # pedantic mode
```

### Common Concepts

| Page | Description |
|------|-------------|
| [Tag Filtering](tag-filtering.md) | Control which strings are included via tags |
| [Encoding](encoding.md) | UTF-8 / UTF-16 output encoding and fallback behavior |
| [Supported Formats](formats.md) | Full reference of all 7 output formats |