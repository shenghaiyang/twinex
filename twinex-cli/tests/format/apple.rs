use twinex_cli::format::Registry;
use twinex_cli::model::{Definition, Lang, Section, SectionName, TwineFile};

use crate::common::*;

#[test]
fn test_apple_format() {
    let tf = make_twine_file();
    let formatter = Registry::get("apple").unwrap();
    let output = fmt_output(&*formatter, "fr", &tf, &default_options());

    assert!(output.contains("Apple Strings File"));
    assert!(output.contains("Language: fr"));
    assert!(output.contains("\"hello\" = \"Bonjour\""));
    assert!(output.contains("\"goodbye\" = \"Au revoir\""));
}

#[test]
fn test_apple_read() {
    let content =
        "/* A greeting */\n\"hello\" = \"Hello\";\n/* A farewell */\n\"goodbye\" = \"Goodbye\";\n";
    let mut tf = TwineFile::new();
    let formatter = Registry::get("apple").unwrap();
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
    assert_eq!(
        tf.sections[0].definitions[0].comment.as_deref(),
        Some("A greeting")
    );
}

#[test]
fn test_apple_detect_language() {
    let formatter = Registry::get("apple").unwrap();
    assert_eq!(
        formatter.detect_language("Resources/en.lproj/Localizable.strings"),
        Some("en".to_string())
    );
    assert_eq!(
        formatter.detect_language("Resources/Base.lproj/Localizable.strings"),
        None
    );
}

#[test]
fn test_apple_output_dir() {
    let formatter = Registry::get("apple").unwrap();
    assert_eq!(formatter.output_dir_for_lang("en"), "en.lproj");
}

#[test]
fn test_apple_roundtrip() {
    let mut tf = TwineFile::new();
    tf.add_language(&Lang::new("en"));
    let mut section = Section::new(SectionName::new("Test"));
    let mut def = Definition::new("hello");
    def.comment = Some("Greeting".to_string());
    def.translations
        .insert(Lang::new("en"), "Hello World".to_string());
    section.definitions.push(def);
    tf.sections.push(section);

    let formatter = Registry::get("apple").unwrap();
    let output = fmt_output(&*formatter, "en", &tf, &default_options());

    let mut tf2 = TwineFile::new();
    formatter.read(&output, "en", &mut tf2).unwrap();

    assert_eq!(tf2.sections[0].definitions.len(), 1);
    assert_eq!(tf2.sections[0].definitions[0].key.as_str(), "hello");
    assert_eq!(
        tf2.sections[0].definitions[0]
            .translations
            .get(&Lang::new("en"))
            .unwrap(),
        "Hello World"
    );
}