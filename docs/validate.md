# Validate

Check that a Twine file is well-formed and optionally enforce naming conventions.

---

## `validate-twine-file`

```sh
twinex validate-twine-file <TWINE_FILE> [OPTIONS]
```

**Examples:**

```sh
# Basic validation
twinex validate-twine-file twine.txt

# Pedantic mode — keys must match ^[A-Za-z0-9_]+$
twinex validate-twine-file twine.txt -p

# With developer language override
twinex validate-twine-file twine.txt -d en
```

---

## Options

| Option | Description |
|--------|-------------|
| `-d, --developer-language` | Override the developer language |
| `-p, --pedantic` | Enforce key naming convention `^[A-Za-z0-9_]+$` |
| `-q, --quiet` | Suppress output |