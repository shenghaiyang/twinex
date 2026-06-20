# Basic Workflow

## 1. Define translations

Create a Twine file with your base language and translations:

```
[[UI]]
    [btn_ok]
        comment = OK button
        en = OK
        zh = 确定
    [btn_cancel]
        comment = Cancel button
        en = Cancel
        zh = 取消

[[Errors]]
    [err_network]
        comment = Network error message
        en = A network error occurred
        zh = 网络错误
```

## 2. Generate platform files

```sh
# iOS
twinex generate-all-localization-files twine.txt Localization/ -f apple -r

# Android
twinex generate-all-localization-files twine.txt res/ -f android -r
```

## 3. Validate

```sh
twinex validate-twine-file twine.txt
```

## Consume existing translations

If translators provide files in platform format, merge them back:

```sh
twinex consume-all-localization-files twine.txt translations/ -f apple
```