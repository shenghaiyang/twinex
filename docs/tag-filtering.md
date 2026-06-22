# Tag Filtering

Tags allow you to control which strings are included in generated output.

---

## Define tags in your Twine file

```
[[UI]]
    [btn_ok]
        tags = ios,android
        en = OK
    [btn_cancel]
        tags = ios
        en = Cancel
    [label_about]
        en = About
```

---

## Filter by tags

```sh
# Only strings tagged "ios"
twinex generate twine.txt output.xml -f android -t ios

# Strings tagged "ios" OR untagged strings
twinex generate twine.txt output.xml -f android -t ios -u
```

---

## How it works

In the example above:

| Key | Tags | `-t ios` | `-t ios -u` |
|-----|------|----------|--------------|
| `btn_ok` | `ios,android` | ✓ included | ✓ included |
| `btn_cancel` | `ios` | ✓ included | ✓ included |
| `label_about` | _(none)_ | ✗ excluded | ✓ included |

- `-t` filters by tag — only strings with matching tags are included
- `-u` includes untagged strings alongside tagged ones