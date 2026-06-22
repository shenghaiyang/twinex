# Quick Start

## Installation

```sh
cargo install twinex-cli
```

## The Twine File Format

A Twine file is a plain text file that stores all your app's translations in one place. It uses indentation to define structure:

```
[[Section Name]]
    [key]
        comment = Developer notes about this key
        tags = ios,android
        ref = another_key
        en = English translation
        zh = Chinese translation
```

| Attribute | Description |
|-----------|-------------|
| `[[Section]]` | Groups related keys together |
| `[key]` | A unique identifier for a translatable string |
| `comment` | Optional note for translators |
| `tags` | Optional comma-separated tags for filtering |
| `ref` | Reference to another key that shares the same base translation |
| `en`, `zh`, etc. | Language code → translation value |

## Example

```sh
# Define translations
cat > twine.txt << 'EOF'
[[Greetings]]
    [hello]
        en = Hello
        zh = 你好
EOF

# Generate iOS strings
twinex generate twine.txt zh.strings -l zh

# Consume back
twinex consume twine.txt zh.strings -l zh
```