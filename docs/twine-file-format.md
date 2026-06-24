## The Twine File Format

A Twine file is a plain text file that stores all your app's translations in one place. It uses indentation to define structure:

```ini
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


!!! info

    More information about the Twine file format can be found in the official Twine specification: [twine-file-format](https://github.com/scelis/twine#twine-file-format)
