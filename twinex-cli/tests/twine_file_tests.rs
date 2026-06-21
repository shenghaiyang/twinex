use std::fs;

use twinex_cli::model::{Lang, TwineFile};

mod common;
use common::*;

#[test]
fn test_read_write_roundtrip() {
    let dir = unique_dir();
    let input_path = write_temp_file(
        &dir,
        "twine.txt",
        "[[General]]\n\
         \t[hello]\n\
         \t\ten = Hello\n\
         \t\tfr = Bonjour\n\
         \t[goodbye]\n\
         \t\ten = Goodbye\n\
         \t\tfr = Au revoir\n",
    );

    let mut tf = TwineFile::new();
    tf.read_from_path(input_path.to_str().unwrap()).unwrap();

    assert_eq!(tf.sections.len(), 1);
    assert_eq!(tf.sections[0].name.as_str(), "General");
    assert_eq!(tf.sections[0].definitions.len(), 2);
    assert_eq!(tf.sections[0].definitions[0].key.as_str(), "hello");
    assert_eq!(tf.sections[0].definitions[1].key.as_str(), "goodbye");
    assert_eq!(
        tf.sections[0].definitions[0]
            .translations
            .get(&Lang::new("en"))
            .unwrap(),
        "Hello"
    );

    let output_path = dir.join("twine_out.txt");
    tf.write_to_path(output_path.to_str().unwrap()).unwrap();

    let mut tf2 = TwineFile::new();
    tf2.read_from_path(output_path.to_str().unwrap()).unwrap();
    assert_eq!(tf2.sections.len(), 1);
    assert_eq!(tf2.sections[0].definitions.len(), 2);
    assert_eq!(
        tf2.sections[0].definitions[0]
            .translations
            .get(&Lang::new("en"))
            .unwrap(),
        "Hello"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_with_tags_and_comments() {
    let dir = unique_dir();
    let input_path = write_temp_file(
        &dir,
        "twine.txt",
        "[[Features]]\n\
         \t[btn_ok]\n\
         \t\ttags = button,ios\n\
         \t\tcomment = OK button text\n\
         \t\ten = OK\n\
         \t[btn_cancel]\n\
         \t\tref = btn_ok\n\
         \t\ttags = button\n\
         \t\ten = Cancel\n",
    );

    let mut tf = TwineFile::new();
    tf.read_from_path(input_path.to_str().unwrap()).unwrap();

    assert_eq!(tf.sections[0].definitions[0].tags, vec!["button", "ios"]);
    assert_eq!(
        tf.sections[0].definitions[0].comment.as_deref(),
        Some("OK button text")
    );
    assert_eq!(
        tf.sections[0].definitions[0]
            .translations
            .get(&Lang::new("en"))
            .unwrap(),
        "OK"
    );
    assert_eq!(
        tf.sections[0].definitions[1]
            .reference_key
            .as_ref()
            .map(|k| k.as_str()),
        Some("btn_ok")
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_language_code_ordering() {
    let mut tf = TwineFile::new();
    tf.add_language(&Lang::new("en"));
    tf.add_language(&Lang::new("zh"));
    tf.add_language(&Lang::new("fr"));
    assert_eq!(tf.language_codes[0].as_str(), "en");

    tf.set_developer_language(&Lang::new("fr"));
    assert_eq!(tf.language_codes[0].as_str(), "fr");
}

#[test]
fn test_parse_error() {
    let dir = unique_dir();
    let input_path = write_temp_file(
        &dir,
        "bad.txt",
        "[[Valid]]\n\t[hello]\n\t\ten = Hello\nbad line here\n",
    );

    let mut tf = TwineFile::new();
    let result = tf.read_from_path(input_path.to_str().unwrap());
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unable to parse"));

    let _ = fs::remove_dir_all(&dir);
}
