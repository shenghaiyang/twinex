use twinex_cli::format::Registry;
use twinex_cli::model::Lang;

use crate::common::*;

#[test]
fn test_django_format() {
    let tf = make_twine_file();
    let formatter = Registry::get("django").unwrap();
    let output = fmt_output(&*formatter, "fr", &tf, &default_options());

    assert!(output.contains("Django Strings File"));
    assert!(output.contains("Language: fr"));
    assert!(output.contains("hello"));
    assert!(output.contains("Bonjour"));
}

#[test]
fn test_django_read() {
    let content = "#. A greeting\nmsgid \"hello\"\nmsgstr \"Bonjour\"\n";
    let mut tf = make_twine_file();
    let formatter = Registry::get("django").unwrap();
    formatter.read(content, "fr", &mut tf).unwrap();

    assert!(!tf.sections.is_empty());
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
