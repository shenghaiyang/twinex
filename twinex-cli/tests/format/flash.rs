use twinex_cli::format::Registry;
use twinex_cli::model::{Lang, TwineFile};

use crate::common::*;

#[test]
fn test_flash_format() {
    let tf = make_twine_file();
    let formatter = Registry::get("flash").unwrap();
    let output = fmt_output(&*formatter, "fr", &tf, &default_options());

    assert!(output.contains("Flash Strings File"));
    assert!(output.contains("Language: fr"));
    assert!(output.contains("hello=Bonjour"));
    assert!(output.contains("goodbye=Au revoir"));
}

#[test]
fn test_flash_read() {
    let content = "# A greeting\nhello=Hello\n# A farewell\ngoodbye=Goodbye\n";
    let mut tf = TwineFile::new();
    let formatter = Registry::get("flash").unwrap();
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
