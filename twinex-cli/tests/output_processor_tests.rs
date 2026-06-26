use twinex_cli::format::{FormatOptions, IncludeMode};
use twinex_cli::model::Lang;
use twinex_cli::output::OutputProcessor;

mod common;
use common::*;

#[test]
fn test_filter_all() {
    let tf = make_twine_file();
    let opts = FormatOptions::default();
    let processor = OutputProcessor::new(&tf, &opts);
    let result = processor.process("fr");
    assert_eq!(result.sections[0].definitions.len(), 2);
}

#[test]
fn test_filter_translated_only() {
    let mut tf = make_twine_file();
    tf.sections[0].definitions[1]
        .translations
        .remove(&Lang::new("fr"));

    let opts = FormatOptions {
        include: IncludeMode::Translated,
        ..Default::default()
    };
    let processor = OutputProcessor::new(&tf, &opts);
    let result = processor.process("fr");
    assert_eq!(result.sections[0].definitions.len(), 1);
    assert_eq!(result.sections[0].definitions[0].key.as_str(), "hello");
}

#[test]
fn test_filter_by_tags() {
    let tf = make_twine_file();
    let opts = FormatOptions {
        tags: vec![vec!["greeting".to_string()]],
        ..Default::default()
    };
    let processor = OutputProcessor::new(&tf, &opts);
    let result = processor.process("en");
    assert_eq!(result.sections[0].definitions.len(), 1);
    assert_eq!(result.sections[0].definitions[0].key.as_str(), "hello");
}

#[test]
fn test_fallback() {
    let mut tf = make_twine_file();
    tf.sections[0].definitions[0]
        .translations
        .remove(&Lang::new("ja"));

    let opts = FormatOptions::default();
    let processor = OutputProcessor::new(&tf, &opts);
    let result = processor.process("ja");
    let hello_def = result.sections[0]
        .definitions
        .iter()
        .find(|d| d.key.as_str() == "hello")
        .unwrap();
    assert_eq!(
        hello_def.translations.get(&Lang::new("ja")).unwrap(),
        "Hello"
    );
}
