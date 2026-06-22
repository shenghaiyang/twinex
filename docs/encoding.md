# Encoding & Fallback

## Encoding

Twinex supports multiple output encodings:

| Encoding | Description |
|----------|-------------|
| `UTF-8` | Default encoding |
| `UTF-16` | UTF-16 with BOM |
| `UTF-16LE` | UTF-16 little-endian |
| `UTF-16BE` | UTF-16 big-endian |

```sh
# Generate iOS strings with UTF-16 encoding
twinex generate twine.txt zh.strings -l zh -e UTF-16
```

---

## Fallback Behavior

When a translation is missing for a language, Twinex falls back to the developer language:

```
[[General]]
    [hello]
        en = Hello       ← developer language
        zh = 你好
        # ja is missing
```

Generating for Japanese:

```
"hello" = "Hello";       ← falls back to English
```

Set the developer language explicitly:

```sh
twinex generate twine.txt zh.strings -l zh -d en
```