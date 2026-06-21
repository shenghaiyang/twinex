use std::collections::HashSet;
use std::fs;

use twinex_cli::format::{FormatOptions, Registry};
use twinex_cli::model::{Definition, Lang, Section, SectionName, TwineFile};

mod common;
use common::*;

#[test]
fn test_generate_apple_and_consume_back() {
    let dir = unique_dir();

    let twine_path = write_temp_file(
        &dir,
        "twine.txt",
        "[[General]]\n\
         \t[hello]\n\
         \t\tcomment = Greeting\n\
         \t\ttags = greeting\n\
         \t\ten = Hello\n\
         \t\tfr = Bonjour\n\
         \t\tja = こんにちは\n",
    );

    let mut tf = TwineFile::new();
    tf.read_from_path(twine_path.to_str().unwrap()).unwrap();
    assert_eq!(tf.language_codes.len(), 3);

    let formatter = Registry::get("apple").unwrap();
    let output = fmt_output(&*formatter, "fr", &tf, &default_options());

    assert!(output.contains("\"hello\" = \"Bonjour\";"));

    let mut tf2 = TwineFile::new();
    tf2.add_language(&Lang::new("en"));
    formatter.read(&output, "fr", &mut tf2).unwrap();

    assert_eq!(tf2.sections[0].definitions[0].key.as_str(), "hello");
    assert_eq!(
        tf2.sections[0].definitions[0]
            .translations
            .get(&Lang::new("fr"))
            .unwrap(),
        "Bonjour"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_generate_android_and_consume_back() {
    let dir = unique_dir();

    let twine_path = write_temp_file(
        &dir,
        "twine.txt",
        "[[App]]\n\
         \t[app_name]\n\
         \t\tcomment = Application name\n\
         \t\ten = MyApp\n\
         \t\tja = マイアプリ\n",
    );

    let mut tf = TwineFile::new();
    tf.read_from_path(twine_path.to_str().unwrap()).unwrap();

    let formatter = Registry::get("android").unwrap();
    let output = fmt_output(&*formatter, "ja", &tf, &default_options());
    assert!(output.contains("app_name"));
    assert!(output.contains("マイアプリ"));

    let mut tf2 = TwineFile::new();
    formatter.read(&output, "ja", &mut tf2).unwrap();
    assert_eq!(tf2.sections[0].definitions[0].key.as_str(), "app_name");
    assert_eq!(
        tf2.sections[0].definitions[0]
            .translations
            .get(&Lang::new("ja"))
            .unwrap(),
        "マイアプリ"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_multiple_formats() {
    let tf = make_twine_file();

    let formats = [
        "apple", "android", "arb", "gettext", "jquery", "django", "flash",
    ];
    for fmt_name in &formats {
        let formatter = Registry::get(fmt_name).unwrap();
        let output = fmt_output(&*formatter, "en", &tf, &default_options());
        assert!(
            !output.is_empty(),
            "Format {} produced empty output",
            fmt_name
        );

        let mut tf2 = TwineFile::new();
        formatter.read(&output, "en", &mut tf2).unwrap();

        assert!(
            !tf2.sections.is_empty(),
            "Format {} read back produced empty sections",
            fmt_name
        );
        assert!(
            !tf2.sections[0].definitions.is_empty(),
            "Format {} read back produced no definitions",
            fmt_name
        );
    }
}

#[test]
fn test_validate_twine_file() {
    let dir = unique_dir();

    let valid_path = write_temp_file(&dir, "valid.txt", "[[Section]]\n\t[key1]\n\t\ten = Value\n");

    let mut tf = TwineFile::new();
    assert!(tf.read_from_path(valid_path.to_str().unwrap()).is_ok());

    let dup_path = write_temp_file(
        &dir,
        "dup.txt",
        "[[Section]]\n\t[key1]\n\t\ten = Value1\n\t[key1]\n\t\ten = Value2\n",
    );
    let mut tf2 = TwineFile::new();
    assert!(tf2.read_from_path(dup_path.to_str().unwrap()).is_ok());

    let all_keys: Vec<_> = tf2.sections[0].definitions.iter().map(|d| &d.key).collect();
    let mut unique = HashSet::new();
    let dupes: Vec<_> = all_keys.iter().filter(|k| !unique.insert(*k)).collect();
    assert!(!dupes.is_empty());

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_tag_filtering() {
    let dir = unique_dir();

    let twine_path = write_temp_file(
        &dir,
        "twine.txt",
        "[[UI]]\n\
         \t[btn_ok]\n\
         \t\ttags = ios,android\n\
         \t\ten = OK\n\
         \t[btn_cancel]\n\
         \t\ttags = ios\n\
         \t\ten = Cancel\n\
         \t[label_untagged]\n\
         \t\ten = No Tags\n",
    );

    let mut tf = TwineFile::new();
    tf.read_from_path(twine_path.to_str().unwrap()).unwrap();

    let options = FormatOptions {
        tags: vec![vec!["ios".to_string()]],
        untagged: false,
        ..Default::default()
    };

    let formatter = Registry::get("apple").unwrap();
    let output = fmt_output(&*formatter, "en", &tf, &options);

    assert!(output.contains("btn_ok"));
    assert!(output.contains("btn_cancel"));
    assert!(!output.contains("label_untagged"));

    let options2 = FormatOptions {
        tags: vec![vec!["ios".to_string()]],
        untagged: true,
        ..Default::default()
    };
    let output2 = fmt_output(&*formatter, "en", &tf, &options2);
    assert!(output2.contains("btn_ok"));
    assert!(output2.contains("btn_cancel"));
    assert!(output2.contains("label_untagged"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_missing_translation_fallback() {
    let dir = unique_dir();

    let twine_path = write_temp_file(
        &dir,
        "twine.txt",
        "[[General]]\n\
         \t[hello]\n\
         \t\ten = Hello\n\
         \t\tfr = Bonjour\n",
    );

    let mut tf = TwineFile::new();
    tf.read_from_path(twine_path.to_str().unwrap()).unwrap();

    let formatter = Registry::get("apple").unwrap();
    let output = fmt_output(&*formatter, "ja", &tf, &default_options());

    assert!(output.contains("\"hello\" = \"Hello\";"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_empty_output_returns_none() {
    let mut tf = TwineFile::new();
    tf.add_language(&Lang::new("en"));
    let section = Section::new(SectionName::new("Empty"));
    tf.sections.push(section);

    let formatter = Registry::get("apple").unwrap();
    assert!(formatter
        .format("en", &tf, &default_options())
        .unwrap()
        .is_none());
}

#[test]
fn test_write_twine_file_with_encoding() {
    let dir = unique_dir();

    let mut tf = TwineFile::new();
    tf.add_language(&Lang::new("en"));
    tf.add_language(&Lang::new("ja"));
    let mut section = Section::new(SectionName::new("Test"));
    let mut def = Definition::new("hello");
    def.comment = Some("A greeting".to_string());
    def.tags = vec!["greeting".to_string()];
    def.translations
        .insert(Lang::new("en"), "Hello".to_string());
    def.translations
        .insert(Lang::new("ja"), "こんにちは".to_string());
    section.definitions.push(def);
    tf.sections.push(section);

    let out_path = dir.join("twine_out.txt");
    tf.write_to_path(out_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&out_path).unwrap();
    assert!(content.contains("[[Test]]"));
    assert!(content.contains("[hello]"));
    assert!(content.contains("tags = greeting"));
    assert!(content.contains("comment = A greeting"));
    assert!(content.contains("en = Hello"));
    assert!(content.contains("ja = こんにちは"));

    let mut tf2 = TwineFile::new();
    tf2.read_from_path(out_path.to_str().unwrap()).unwrap();
    assert_eq!(tf2.language_codes[0].as_str(), "en");
    assert_eq!(tf2.sections[0].definitions[0].tags, vec!["greeting"]);

    let _ = fs::remove_dir_all(&dir);
}
