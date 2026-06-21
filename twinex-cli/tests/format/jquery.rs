use twinex_cli::format::Registry;
use twinex_cli::model::{Definition, Lang, Section, SectionName, TwineFile};

use crate::common::*;

#[test]
fn test_jquery_format() {
    let mut tf = TwineFile::new();
    tf.add_language(&Lang::new("en"));
    let mut section = Section::new(SectionName::new(""));
    let mut def = Definition::new("hello");
    def.translations
        .insert(Lang::new("en"), "Hello".to_string());
    section.definitions.push(def);
    tf.sections.push(section);

    let formatter = Registry::get("jquery").unwrap();
    let output = fmt_output(&*formatter, "en", &tf, &default_options());

    assert!(output.starts_with("{"));
    assert!(output.ends_with("}\n"));
    assert!(output.contains("hello"));
    assert!(output.contains("Hello"));
}

#[test]
fn test_jquery_read() {
    let content = "{\"hello\":\"Hello\",\"goodbye\":\"Goodbye\"}";
    let mut tf = TwineFile::new();
    let formatter = Registry::get("jquery").unwrap();
    formatter.read(content, "en", &mut tf).unwrap();

    assert_eq!(tf.sections[0].definitions.len(), 2);
    assert_eq!(tf.sections[0].definitions[0].key.as_str(), "hello");
    assert_eq!(
        tf.sections[0].definitions[0]
            .translations
            .get(&Lang::new("en"))
            .unwrap(),
        "Hello"
    );
}

#[test]
fn test_jquery_detect_language() {
    let formatter = Registry::get("jquery").unwrap();
    assert_eq!(
        formatter.detect_language("localize-fr.json"),
        Some("fr".to_string())
    );
}
