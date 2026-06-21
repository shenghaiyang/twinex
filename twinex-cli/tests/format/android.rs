use twinex_cli::format::Registry;
use twinex_cli::model::{Definition, Lang, Section, SectionName, TwineFile};

use crate::common::*;

#[test]
fn test_android_format() {
    let tf = make_twine_file();
    let formatter = Registry::get("android").unwrap();
    let output = fmt_output(&*formatter, "fr", &tf, &default_options());

    assert!(output.contains("<?xml version=\"1.0\""));
    assert!(output.contains("Language: fr"));
    assert!(output.contains("<resources>"));
    assert!(output.contains("</resources>"));
    assert!(output.contains("hello"));
    assert!(output.contains("Bonjour"));
}

#[test]
fn test_android_read() {
    let content = r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <!-- A greeting -->
    <string name="hello">Hello</string>
    <string name="goodbye">Goodbye</string>
</resources>"#;

    let mut tf = TwineFile::new();
    let formatter = Registry::get("android").unwrap();
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
fn test_android_detect_language() {
    let formatter = Registry::get("android").unwrap();
    assert_eq!(
        formatter.detect_language("res/values-fr/strings.xml"),
        Some("fr".to_string())
    );
    assert_eq!(formatter.detect_language("res/values/strings.xml"), None);
}

#[test]
fn test_android_output_dir() {
    let formatter = Registry::get("android").unwrap();
    assert_eq!(formatter.output_dir_for_lang(""), "values");
    assert_eq!(formatter.output_dir_for_lang("fr"), "values-fr");
    assert_eq!(formatter.output_dir_for_lang("en-GB"), "values-en-rGB");
}

#[test]
fn test_android_read_with_html_entities() {
    let content = r#"<string name="welcome">Welcome to &lt;b&gt;Android&lt;/b&gt;</string>"#;
    let mut tf = TwineFile::new();
    let formatter = Registry::get("android").unwrap();
    formatter.read(content, "en", &mut tf).unwrap();

    assert_eq!(
        tf.sections[0].definitions[0]
            .translations
            .get(&Lang::new("en"))
            .unwrap(),
        "Welcome to <b>Android</b>"
    );
}

#[test]
fn test_android_roundtrip() {
    let mut tf = TwineFile::new();
    tf.add_language(&Lang::new("en"));
    let mut section = Section::new(SectionName::new("Test"));
    let mut def = Definition::new("app_name");
    def.comment = Some("App name".to_string());
    def.translations
        .insert(Lang::new("en"), "MyApp".to_string());
    section.definitions.push(def);
    tf.sections.push(section);

    let formatter = Registry::get("android").unwrap();
    let output = fmt_output(&*formatter, "en", &tf, &default_options());

    let mut tf2 = TwineFile::new();
    formatter.read(&output, "en", &mut tf2).unwrap();

    assert_eq!(tf2.sections[0].definitions[0].key.as_str(), "app_name");
    assert_eq!(
        tf2.sections[0].definitions[0]
            .translations
            .get(&Lang::new("en"))
            .unwrap(),
        "MyApp"
    );
}
