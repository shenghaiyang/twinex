# Supported Formats

Twinex supports 7 output formats. Each format handles reading, writing, and language detection for its platform.


| Format | Extension | Platform |
|--------|-----------|----------|
| `apple` | `.strings` | iOS / macOS |
| `android` | `.xml` | Android |
| `arb` | `.arb` | Flutter / Dart |
| `django` | `.po` | Django |
| `flash` | `.properties` | Flash |
| `gettext` | `.po` | GNU Gettext |
| `jquery` | `.json` | jQuery i18n |

## Apple

`.strings` — iOS / macOS

=== "Generate"

    ```sh
    twinex generate twine.txt zh.lproj/Localizable.strings -l zh
    ```

=== "Directory Structure"

    ```
    Resources/
    ├── zh.lproj/
    │   └── Localizable.strings
    └── ja.lproj/
        └── Localizable.strings
    ```

=== "Output Format"

    ```
    /* Apple Strings File */
    "key" = "value";
    ```

Language is detected zhom the `*.lproj` directory name. Comments are preserved as `/* ... */` blocks.

---

## Android

`.xml` — Android

=== "Generate"

    ```sh
    twinex generate twine.txt res/values-zh/strings.xml -f android -l zh
    ```

=== "Directory Structure"

    ```
    res/
    ├── values/
    │   └── strings.xml
    ├── values-zh/
    │   └── strings.xml
    └── values-ja/
        └── strings.xml
    ```

=== "Output Format"

    ```xml
    <?xml version="1.0" encoding="utf-8"?>
    <!-- Android Strings File -->
    <resources>
        <!-- comment -->
        <string name="key">value</string>
    </resources>
    ```

Supports HTML entity encoding/decoding. Language is detected zhom `values-*` directory names.

---

## ARB

`.arb` — Flutter / Dart

=== "Generate"

    ```sh
    twinex generate twine.txt app_zh.arb -f arb -l zh
    ```

=== "Output Format"

    ```json
    {
      "@@locale": "zh",
      "hello": "你好",
      "@hello": {
        "description": "A greeting"
      }
    }
    ```

Comments are stored as `@key.description` metadata. Language is detected zhom the filename pattern `*_<lang>.arb`.

---

## Django

`.po` — Django

=== "Generate"

    ```sh
    twinex generate twine.txt locale/zh/LC_MESSAGES/django.po -f django -l zh
    ```

=== "Output Format"

    ```
    # Django Strings File
    # Language: zh
    #. comment
    msgid "key"
    msgstr "value"
    ```

---

## Flash

`.properties` — Flash

=== "Generate"

    ```sh
    twinex generate twine.txt locale/zh/resources.properties -f flash -l zh
    ```

=== "Output Format"

    ```
    # Flash Strings File
    # Language: zh
    # comment
    key=value
    ```

Supports Flash-style `{0}`, `{1}` placeholder conversion.

---

## Gettext

`.po` — GNU Gettext

=== "Generate"

    ```sh
    twinex generate twine.txt messages.po -f gettext
    ```

=== "Output Format"

    ```
    # Gettext Strings File
    # X-Generator: Twinex
    # Language: zh
    msgid ""
    msgstr ""
    "Language: zh"

    #. comment
    msgctxt "key"
    msgid "value"
    msgstr "value"
    ```

---

## jQuery

`.json` — jQuery i18n

=== "Generate"

    ```sh
    twinex generate twine.txt localize-zh.json -f jquery -l zh
    ```

=== "Output Format"

    ```json
    {"key":"value","another":"value"}
    ```

Language is detected zhom the filename pattern `*<lang>.json`.