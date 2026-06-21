use twinex_cli::model::{Definition, Lang};

#[test]
fn test_matches_tags_no_filter() {
    let def = Definition::new("key");
    assert!(def.matches_tags(&[], false));
}

#[test]
fn test_matches_tags_with_untagged() {
    let mut def = Definition::new("key");
    def.tags = vec!["greeting".to_string()];
    assert!(def.matches_tags(&[vec!["greeting".to_string()]], false));
    assert!(!def.matches_tags(&[vec!["other".to_string()]], false));
    assert!(!def.matches_tags(&[vec!["other".to_string()]], true));
}

#[test]
fn test_matches_tags_untagged_definition() {
    let def = Definition::new("key");
    assert!(!def.matches_tags(&[vec!["greeting".to_string()]], false));
    assert!(def.matches_tags(&[vec!["greeting".to_string()]], true));
}

#[test]
fn test_matches_tags_negated() {
    let mut def = Definition::new("key");
    def.tags = vec!["greeting".to_string()];
    assert!(def.matches_tags(&[vec!["~farewell".to_string()]], false));
    assert!(!def.matches_tags(&[vec!["~greeting".to_string()]], false));
}

#[test]
fn test_translation_for_lang() {
    let mut def = Definition::new("key");
    def.translations
        .insert(Lang::new("en"), "Hello".to_string());
    def.translations
        .insert(Lang::new("fr"), "Bonjour".to_string());

    assert_eq!(def.translation_for(&[Lang::new("en")]), Some("Hello"));
    assert_eq!(def.translation_for(&[Lang::new("de")]), None);
    assert_eq!(
        def.translation_for(&[Lang::new("de"), Lang::new("fr")]),
        Some("Bonjour")
    );
}
