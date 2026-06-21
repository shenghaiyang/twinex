#![allow(dead_code)]

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

use twinex_cli::format::FormatOptions;
use twinex_cli::model::{Definition, Lang, Section, SectionName, TwineFile};

static COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn unique_dir() -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("twinex_test_{}_{}", std::process::id(), id));
    fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn write_temp_file(dir: &PathBuf, name: &str, content: &str) -> PathBuf {
    let path = dir.join(name);
    let mut f = fs::File::create(&path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
    path
}

pub fn make_twine_file() -> TwineFile {
    let mut tf = TwineFile::new();
    tf.add_language(&Lang::new("en"));
    tf.add_language(&Lang::new("fr"));
    tf.add_language(&Lang::new("ja"));

    let mut section = Section::new(SectionName::new("General"));

    let mut def1 = Definition::new("hello");
    def1.comment = Some("A greeting".to_string());
    def1.tags = vec!["greeting".to_string()];
    def1.translations
        .insert(Lang::new("en"), "Hello".to_string());
    def1.translations
        .insert(Lang::new("fr"), "Bonjour".to_string());
    def1.translations
        .insert(Lang::new("ja"), "こんにちは".to_string());
    section.definitions.push(def1);

    let mut def2 = Definition::new("goodbye");
    def2.comment = Some("A farewell".to_string());
    def2.tags = vec!["farewell".to_string()];
    def2.translations
        .insert(Lang::new("en"), "Goodbye".to_string());
    def2.translations
        .insert(Lang::new("fr"), "Au revoir".to_string());
    section.definitions.push(def2);

    tf.sections.push(section);
    tf
}

pub fn default_options() -> FormatOptions {
    FormatOptions::default()
}

/// Helper: format and unwrap both Result and Option layers (for tests that expect non-empty output).
pub fn fmt_output(
    formatter: &dyn twinex_cli::format::Formatter,
    lang: &str,
    tf: &TwineFile,
    opts: &FormatOptions,
) -> String {
    formatter.format(lang, tf, opts).unwrap().unwrap()
}
