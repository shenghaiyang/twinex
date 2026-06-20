pub mod android;
pub mod apple;
pub mod arb;
pub mod django;
pub mod flash;
pub mod gettext;
pub mod jquery;

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::model::{Definition, Lang, SectionName, TwineFile};

/// The core trait that every localization format must implement.
pub trait Formatter: Send + Sync {
    /// Human-readable format name (e.g. "apple", "android").
    fn name(&self) -> &'static str;

    /// File extension (e.g. ".strings", ".xml").
    fn extension(&self) -> &'static str;

    /// Default file name when generating multiple files.
    fn default_file_name(&self) -> &'static str {
        "strings.txt"
    }

    /// Whether this formatter can handle the given directory structure.
    fn can_handle_directory(&self, _path: &str) -> bool {
        false
    }

    /// Try to extract the language code from a path.
    fn detect_language(&self, _path: &str) -> Option<String> {
        None
    }

    /// Output directory name for a given language.
    fn output_dir_for_lang(&self, _lang: &str) -> String {
        String::new()
    }

    /// Read translations from file content into a TwineFile.
    fn read(&self, content: &str, lang: &str, twine_file: &mut TwineFile) -> crate::error::Result<()>;

    /// Format a TwineFile into this format's output for a given language.
    fn format(
        &self,
        lang: &str,
        twine_file: &TwineFile,
        options: &FormatOptions,
    ) -> crate::error::Result<Option<String>>;
}

// ── Format options ────────────────────────────────────────────

/// Options controlling how a formatter generates output.
#[derive(Debug, Clone)]
pub struct FormatOptions {
    /// Filter by tags (AND of OR groups).
    pub tags: Vec<Vec<String>>,
    /// Include untagged definitions.
    pub untagged: bool,
    /// Which translations to include: "all", "translated", "untranslated".
    pub include: IncludeMode,
    /// Override the developer language.
    pub developer_language: Option<String>,
    /// Escape all HTML tags (Android-specific).
    pub escape_all_tags: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum IncludeMode {
    #[default]
    All,
    Translated,
    Untranslated,
}

impl IncludeMode {
    pub fn from_str(s: &str) -> Self {
        match s {
            "translated" => IncludeMode::Translated,
            "untranslated" => IncludeMode::Untranslated,
            _ => IncludeMode::All,
        }
    }
}

impl Default for FormatOptions {
    fn default() -> Self {
        FormatOptions {
            tags: Vec::new(),
            untagged: false,
            include: IncludeMode::All,
            developer_language: None,
            escape_all_tags: false,
        }
    }
}

impl FormatOptions {
    pub fn builder() -> FormatOptionsBuilder {
        FormatOptionsBuilder::default()
    }
}

/// Builder for `FormatOptions`.
#[derive(Default)]
pub struct FormatOptionsBuilder {
    tags: Vec<Vec<String>>,
    untagged: bool,
    include: IncludeMode,
    developer_language: Option<String>,
    escape_all_tags: bool,
}

impl FormatOptionsBuilder {
    pub fn tags(mut self, tags: Vec<Vec<String>>) -> Self {
        self.tags = tags;
        self
    }
    pub fn untagged(mut self, yes: bool) -> Self {
        self.untagged = yes;
        self
    }
    pub fn include(mut self, mode: IncludeMode) -> Self {
        self.include = mode;
        self
    }
    pub fn developer_language(mut self, lang: Option<String>) -> Self {
        self.developer_language = lang;
        self
    }
    pub fn escape_all_tags(mut self, yes: bool) -> Self {
        self.escape_all_tags = yes;
        self
    }
    pub fn build(self) -> FormatOptions {
        FormatOptions {
            tags: self.tags,
            untagged: self.untagged,
            include: self.include,
            developer_language: self.developer_language,
            escape_all_tags: self.escape_all_tags,
        }
    }
}

// ── Registry ──────────────────────────────────────────────────

type FormatterFactory = fn() -> Box<dyn Formatter>;

/// Global registry of formatters. External crates can register their own formatters.
pub struct Registry {
    factories: HashMap<String, FormatterFactory>,
}

impl Registry {
    fn global() -> &'static Registry {
        static REGISTRY: OnceLock<Registry> = OnceLock::new();
        REGISTRY.get_or_init(|| {
            let mut reg = Registry {
                factories: HashMap::new(),
            };
            reg.register("apple", || Box::new(crate::format::apple::AppleFormatter));
            reg.register("android", || Box::new(crate::format::android::AndroidFormatter));
            reg.register("arb", || Box::new(crate::format::arb::ArbFormatter));
            reg.register("gettext", || Box::new(crate::format::gettext::GettextFormatter));
            reg.register("jquery", || Box::new(crate::format::jquery::JQueryFormatter));
            reg.register("django", || Box::new(crate::format::django::DjangoFormatter));
            reg.register("flash", || Box::new(crate::format::flash::FlashFormatter));
            reg
        })
    }

    /// Register a formatter. Panics if the name is already taken.
    pub fn register(&mut self, name: &str, factory: FormatterFactory) {
        if self.factories.contains_key(name) {
            panic!("Formatter '{}' is already registered", name);
        }
        self.factories.insert(name.to_string(), factory);
    }

    /// Get a formatter by name.
    pub fn get(name: &str) -> Option<Box<dyn Formatter>> {
        Self::global().factories.get(name).map(|f| f())
    }

    /// List all registered formatter names.
    pub fn names() -> Vec<&'static str> {
        Self::global().factories.keys().map(|s| s.as_str()).collect()
    }

    /// Find a formatter by extension.
    pub fn by_extension(ext: &str) -> Option<Box<dyn Formatter>> {
        Self::global()
            .factories
            .values()
            .map(|f| f())
            .find(|f| f.extension() == ext)
    }

    /// Find a formatter that can handle the given directory.
    pub fn by_directory(path: &str) -> Option<Box<dyn Formatter>> {
        let candidates: Vec<_> = Self::global()
            .factories
            .values()
            .map(|f| f())
            .filter(|f| f.can_handle_directory(path))
            .collect();
        if candidates.len() == 1 {
            candidates.into_iter().next()
        } else {
            None
        }
    }

    /// Find a formatter by extension, or by directory, or by name.
    pub fn detect(name: Option<&str>, path: Option<&str>) -> Option<Box<dyn Formatter>> {
        if let Some(n) = name {
            return Self::get(n);
        }
        if let Some(p) = path {
            if let Some(ext) = std::path::Path::new(p).extension().and_then(|e| e.to_str()) {
                let ext = format!(".{}", ext);
                if let Some(f) = Self::by_extension(&ext) {
                    return Some(f);
                }
            }
            if let Some(f) = Self::by_directory(p) {
                return Some(f);
            }
        }
        None
    }
}

// ── Common helpers ────────────────────────────────────────────

/// Find or create a definition and set its translation.
pub fn set_translation(twine_file: &mut TwineFile, key: &str, lang: &str, value: &str) {
    use crate::model::Section;
    for section in &mut twine_file.sections {
        for def in &mut section.definitions {
            if def.key.as_str() == key {
                def.translations.insert(Lang::new(lang), value.to_string());
                return;
            }
        }
    }
    if twine_file.sections.is_empty() {
        twine_file.sections.push(Section::new(SectionName::new("")));
    }
    let mut def = Definition::new(key);
    def.translations.insert(Lang::new(lang), value.to_string());
    twine_file.sections[0].definitions.push(def);
}

/// Find a definition and set its comment.
pub fn set_comment(twine_file: &mut TwineFile, key: &str, comment: &str) {
    for section in &mut twine_file.sections {
        for def in &mut section.definitions {
            if def.key.as_str() == key {
                def.comment = Some(comment.to_string());
                return;
            }
        }
    }
}

/// Escape double quotes for PO-like formats.
pub fn escape_quotes(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}