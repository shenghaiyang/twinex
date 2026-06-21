use twinex_cli::format::Registry;
use twinex_cli::model::Lang;

use crate::common::*;

#[test]
fn test_gettext_format() {
    let tf = make_twine_file();
    let formatter = Registry::get("gettext").unwrap();
    let output = fmt_output(&*formatter, "fr", &tf, &default_options());

    assert!(output.contains("X-Generator: Twinex"));
    assert!(output.contains("Language: fr"));
    assert!(output.contains("hello"));
    assert!(output.contains("Bonjour"));
}

#[test]
fn test_gettext_read() {
    let content = "msgid \"\"\nmsgstr \"\"\n\"Language: fr\"\n\n#. \"A greeting\"\nmsgctxt \"hello\"\nmsgid \"Hello\"\nmsgstr \"Bonjour\"\n";
    let mut tf = make_twine_file();
    let formatter = Registry::get("gettext").unwrap();
    formatter.read(content, "fr", &mut tf).unwrap();

    assert!(!tf.sections.is_empty());
    assert!(!tf.sections[0].definitions.is_empty());
    assert_eq!(tf.sections[0].definitions[0].key.as_str(), "hello");
    assert_eq!(
        tf.sections[0].definitions[0]
            .translations
            .get(&Lang::new("fr"))
            .unwrap(),
        "Bonjour"
    );
    assert_eq!(
        tf.sections[0].definitions[0].comment.as_deref(),
        Some("A greeting")
    );
}
