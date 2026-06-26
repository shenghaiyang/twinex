use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{Result, TwinexError};

// ── Newtype wrappers ──────────────────────────────────────────

/// A localization key identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Key(String);

impl Key {
    pub fn new(s: impl Into<String>) -> Self {
        Key(s.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        Key(s.to_string())
    }
}

/// A language code (e.g. "en", "fr", "zh-Hans").
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct Lang(String);

impl Lang {
    pub fn new(s: impl Into<String>) -> Self {
        Lang(s.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<&str> for Lang {
    fn from(s: &str) -> Self {
        Lang(s.to_string())
    }
}

/// A section name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SectionName(String);

impl SectionName {
    pub fn new(s: impl Into<String>) -> Self {
        SectionName(s.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SectionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

// ── Definition ────────────────────────────────────────────────

/// A single localization entry with its translations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    pub key: Key,
    pub comment: Option<String>,
    pub tags: Vec<String>,
    pub translations: HashMap<Lang, String>,
    pub reference_key: Option<Key>,
}

impl Definition {
    pub fn new(key: impl Into<Key>) -> Self {
        Definition {
            key: key.into(),
            comment: None,
            tags: Vec::new(),
            translations: HashMap::new(),
            reference_key: None,
        }
    }

    /// Fetch the translation for `lang`, falling back through `fallback_langs`.
    pub fn translation_for(&self, lang: &[Lang]) -> Option<&str> {
        for l in lang {
            if let Some(t) = self.translations.get(l) {
                return Some(t.as_str());
            }
        }
        None
    }

    /// Check whether this definition matches the given tag criteria.
    ///
    /// `tags`: outer Vec = AND, inner Vec = OR. Tags prefixed with `~` are negated.
    /// `include_untagged`: whether untagged definitions pass the filter.
    pub fn matches_tags(&self, tags: &[Vec<String>], include_untagged: bool) -> bool {
        if tags.is_empty() {
            return true;
        }
        if self.tags.is_empty() {
            return include_untagged;
        }
        tags.iter().all(|tag_set| {
            let mut has_positive = false;
            let mut matches_positive = false;
            let mut has_negated = false;
            let mut matches_negative = true;
            for t in tag_set {
                if let Some(neg) = t.strip_prefix('~') {
                    has_negated = true;
                    if self.tags.contains(&neg.to_string()) {
                        matches_negative = false;
                    }
                } else {
                    has_positive = true;
                    if self.tags.contains(t) {
                        matches_positive = true;
                    }
                }
            }
            (has_positive && matches_positive) || (has_negated && matches_negative)
        })
    }
}

// ── Section ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub name: SectionName,
    pub definitions: Vec<Definition>,
}

impl Section {
    pub fn new(name: impl Into<SectionName>) -> Self {
        Section {
            name: name.into(),
            definitions: Vec::new(),
        }
    }
}

// ── TwineFile ─────────────────────────────────────────────────

/// The parsed representation of a Twine data file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwineFile {
    pub sections: Vec<Section>,
    pub language_codes: Vec<Lang>,
}

impl TwineFile {
    pub fn new() -> Self {
        TwineFile {
            sections: Vec::new(),
            language_codes: Vec::new(),
        }
    }

    /// Add a language code, keeping the developer language (first) in place.
    pub fn add_language(&mut self, code: &Lang) {
        if self.language_codes.is_empty() {
            self.language_codes.push(code.clone());
        } else if !self.language_codes.contains(code) {
            let dev = self.language_codes[0].clone();
            self.language_codes.push(code.clone());
            self.language_codes.retain(|l| l != &dev);
            self.language_codes.sort();
            self.language_codes.insert(0, dev);
        }
    }

    /// Set the developer language (moves it to position 0).
    pub fn set_developer_language(&mut self, code: &Lang) {
        self.language_codes.retain(|l| l != code);
        self.language_codes.insert(0, code.clone());
    }

    /// Parse a Twine data file from disk.
    pub fn read_from_path<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        let file = fs::File::open(path).map_err(|e| {
            TwinexError::Io(std::io::Error::other(format!("{}: {}", path.display(), e)))
        })?;
        let reader = BufReader::new(file);
        self.read(reader, &path.display().to_string())
    }

    /// Parse a Twine data file from any `BufRead`.
    pub fn read<R: BufRead>(&mut self, reader: R, path_hint: &str) -> Result<()> {
        let mut has_section = false;

        for (line_num, line_result) in reader.lines().enumerate() {
            let line = line_result?;
            let line = line.trim();
            let line_num = line_num + 1;

            if line.is_empty() {
                continue;
            }

            let mut parsed = false;

            if line.len() > 4 && line.starts_with("[[") {
                // Section header: [[SectionName]]
                if let Some(caps) = RegexStore::section().captures(line) {
                    has_section = true;
                    self.sections
                        .push(Section::new(SectionName(caps[1].to_string())));
                    parsed = true;
                }
            } else if line.len() > 2 && line.starts_with('[') {
                // Key definition: [key]
                if let Some(caps) = RegexStore::key().captures(line) {
                    if !has_section {
                        has_section = true;
                        self.sections.push(Section::new(SectionName(String::new())));
                    }
                    let section = self.sections.last_mut().unwrap();
                    section
                        .definitions
                        .push(Definition::new(Key::new(&caps[1])));
                    parsed = true;
                }
            } else {
                // Key-value pair: k = v
                if let Some(caps) = RegexStore::key_value().captures(line) {
                    let key = caps[1].trim().to_string();
                    let mut value = caps[2].trim().to_string();

                    if value.starts_with('`') && value.ends_with('`') {
                        value = value[1..value.len() - 1].to_string();
                    }

                    // Use index-based access to avoid borrow conflicts
                    let sec_idx =
                        self.sections
                            .len()
                            .checked_sub(1)
                            .ok_or_else(|| TwinexError::Parse {
                                path: path_hint.to_string(),
                                line: line_num,
                                message: "No definition to attach property to".to_string(),
                            })?;
                    let def_idx = self.sections[sec_idx]
                        .definitions
                        .len()
                        .checked_sub(1)
                        .ok_or_else(|| TwinexError::Parse {
                            path: path_hint.to_string(),
                            line: line_num,
                            message: "No definition to attach property to".to_string(),
                        })?;

                    // Handle non-translation keys first
                    if key == "comment" {
                        self.sections[sec_idx].definitions[def_idx].comment = Some(value);
                    } else if key == "tags" {
                        self.sections[sec_idx].definitions[def_idx].tags =
                            value.split(',').map(|s| s.trim().to_string()).collect();
                    } else if key == "ref" {
                        if !value.is_empty() {
                            self.sections[sec_idx].definitions[def_idx].reference_key =
                                Some(Key::new(value));
                        }
                    } else {
                        // Translation key
                        let lang = Lang::new(&key);
                        if !self.language_codes.contains(&lang) {
                            self.add_language(&lang);
                        }
                        self.sections[sec_idx].definitions[def_idx]
                            .translations
                            .insert(lang, value);
                    }
                    parsed = true;
                }
            }

            if !parsed {
                return Err(TwinexError::Parse {
                    path: path_hint.to_string(),
                    line: line_num,
                    message: format!("Unable to parse: {}", line),
                });
            }
        }
        Ok(())
    }

    /// Write the Twine data file to disk.
    pub fn write_to_path<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let mut file = fs::File::create(path).map_err(|e| {
            TwinexError::Io(std::io::Error::other(format!(
                "Cannot write {}: {}",
                path.display(),
                e
            )))
        })?;
        self.write(&mut file)
    }

    /// Write the Twine data file to any `Write`.
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let dev_lang = self.language_codes.first();

        for section in &self.sections {
            writeln!(writer, "\n[[{}]]", section.name).ok();

            for def in &section.definitions {
                writeln!(writer, "\t[{}]", def.key).ok();

                if let Some(ref ref_key) = def.reference_key {
                    writeln!(writer, "\t\tref = {}", ref_key).ok();
                }
                if !def.tags.is_empty() {
                    writeln!(writer, "\t\ttags = {}", def.tags.join(",")).ok();
                }
                if let Some(ref comment) = def.comment {
                    if !comment.is_empty() {
                        writeln!(writer, "\t\tcomment = {}", comment).ok();
                    }
                }

                if let Some(dev) = dev_lang {
                    if let Some(value) = def.translations.get(dev) {
                        write_value(writer, dev, value);
                    }
                }
                for lang in self.language_codes.iter().skip(1) {
                    if let Some(value) = def.translations.get(lang) {
                        write_value(writer, lang, value);
                    }
                }
            }
        }
        writer.flush().map_err(TwinexError::from)
    }
}

impl Default for TwineFile {
    fn default() -> Self {
        Self::new()
    }
}

// ── Helpers ───────────────────────────────────────────────────

fn write_value(w: &mut impl Write, lang: &Lang, value: &str) {
    let formatted = if value.starts_with(' ')
        || value.ends_with(' ')
        || (value.starts_with('`') && value.ends_with('`'))
    {
        format!("`{}`", value)
    } else {
        value.to_string()
    };
    writeln!(w, "\t\t{} = {}", lang, formatted).ok();
}

// ── Regex store ───────────────────────────────────────────────

struct RegexStore;

impl RegexStore {
    fn section() -> &'static regex::Regex {
        use std::sync::OnceLock;
        static RE: OnceLock<regex::Regex> = OnceLock::new();
        RE.get_or_init(|| regex::Regex::new(r"^\[\[(.+)\]\]$").unwrap())
    }
    fn key() -> &'static regex::Regex {
        use std::sync::OnceLock;
        static RE: OnceLock<regex::Regex> = OnceLock::new();
        RE.get_or_init(|| regex::Regex::new(r"^\[(.+)\]$").unwrap())
    }
    fn key_value() -> &'static regex::Regex {
        use std::sync::OnceLock;
        static RE: OnceLock<regex::Regex> = OnceLock::new();
        RE.get_or_init(|| regex::Regex::new(r"^([^=]+)=(.*)$").unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_definition_matches_tags() {
        let mut def = Definition::new(Key::new("test"));
        def.tags = vec!["greeting".to_string()];
        assert!(def.matches_tags(&[vec!["greeting".to_string()]], false));
        assert!(!def.matches_tags(&[vec!["other".to_string()]], false));
    }

    #[test]
    fn test_definition_translation_fallback() {
        let mut def = Definition::new(Key::new("key"));
        def.translations
            .insert(Lang::new("en"), "Hello".to_string());
        def.translations
            .insert(Lang::new("fr"), "Bonjour".to_string());
        assert_eq!(
            def.translation_for(&[Lang::new("de"), Lang::new("en")]),
            Some("Hello")
        );
        assert_eq!(def.translation_for(&[Lang::new("de")]), None);
    }

    #[test]
    fn test_twine_file_roundtrip() {
        let input = "[[Greetings]]\n\t[hello]\n\t\ten = Hello\n\t\tfr = Bonjour\n";
        let mut tf = TwineFile::new();
        tf.read(input.as_bytes(), "test").unwrap();

        assert_eq!(tf.sections.len(), 1);
        assert_eq!(tf.sections[0].definitions.len(), 1);
        assert_eq!(tf.sections[0].definitions[0].key.as_str(), "hello");

        let mut output = Vec::new();
        tf.write(&mut output).unwrap();

        let mut tf2 = TwineFile::new();
        tf2.read(output.as_slice(), "test").unwrap();
        assert_eq!(tf2.sections[0].definitions[0].key.as_str(), "hello");
        assert_eq!(
            tf2.sections[0].definitions[0]
                .translations
                .get(&Lang::new("en"))
                .unwrap(),
            "Hello"
        );
    }
}
