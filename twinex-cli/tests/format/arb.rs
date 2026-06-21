use twinex_cli::format::Registry;
use twinex_cli::model::{Definition, Lang, Section, SectionName, TwineFile};

use crate::common::*;

#[test]
fn test_arb_format() {
    let tf = make_twine_file();
    let formatter = Registry::get("arb").unwrap();
    let output = fmt_output(&*formatter, "fr", &tf, &default_options());

    assert!(output.contains("\"@@locale\": \"fr\""));
    assert!(output.contains("\"hello\": \"Bonjour\""));
    assert!(output.contains("\"goodbye\": \"Au revoir\""));
    assert!(output.contains("\"@hello\""));
    assert!(output.contains("\"description\": \"A greeting\""));
}

#[test]
fn test_arb_read() {
    let content = r#"{
  "@@locale": "en",
  "hello": "Hello",
  "@hello": {
    "description": "A greeting"
  },
  "goodbye": "Goodbye"
}"#;
    let mut tf = TwineFile::new();
    let formatter = Registry::get("arb").unwrap();
    formatter.read(content, "en", &mut tf).unwrap();

    assert_eq!(tf.sections[0].definitions.len(), 2);

    let hello_def = tf.sections[0]
        .definitions
        .iter()
        .find(|d| d.key.as_str() == "hello")
        .unwrap();
    assert_eq!(
        hello_def.translations.get(&Lang::new("en")).unwrap(),
        "Hello"
    );
    assert_eq!(hello_def.comment.as_deref(), Some("A greeting"));
}

#[test]
fn test_arb_detect_language() {
    let formatter = Registry::get("arb").unwrap();
    assert_eq!(
        formatter.detect_language("intl_fr.arb"),
        Some("fr".to_string())
    );
    assert_eq!(
        formatter.detect_language("app_en-GB.arb"),
        Some("en-GB".to_string())
    );
}

#[test]
fn test_arb_roundtrip() {
    let mut tf = TwineFile::new();
    tf.add_language(&Lang::new("en"));
    let mut section = Section::new(SectionName::new("Test"));
    let mut def = Definition::new("welcome");
    def.comment = Some("Welcome message".to_string());
    def.translations
        .insert(Lang::new("en"), "Hello World".to_string());
    section.definitions.push(def);
    tf.sections.push(section);

    let formatter = Registry::get("arb").unwrap();
    let output = fmt_output(&*formatter, "en", &tf, &default_options());

    let mut tf2 = TwineFile::new();
    formatter.read(&output, "en", &mut tf2).unwrap();

    assert_eq!(tf2.sections[0].definitions.len(), 1);
    assert_eq!(tf2.sections[0].definitions[0].key.as_str(), "welcome");
    assert_eq!(
        tf2.sections[0].definitions[0]
            .translations
            .get(&Lang::new("en"))
            .unwrap(),
        "Hello World"
    );
    assert_eq!(
        tf2.sections[0].definitions[0].comment.as_deref(),
        Some("Welcome message")
    );
}
