use twinex_cli::format::Registry;

#[test]
fn test_names() {
    let all = Registry::names();
    assert_eq!(all.len(), 7);
    assert!(all.contains(&"apple"));
    assert!(all.contains(&"android"));
    assert!(all.contains(&"arb"));
    assert!(all.contains(&"gettext"));
    assert!(all.contains(&"jquery"));
    assert!(all.contains(&"django"));
    assert!(all.contains(&"flash"));
}

#[test]
fn test_by_extension() {
    let apple = Registry::by_extension(".strings").unwrap();
    assert_eq!(apple.name(), "apple");

    let android = Registry::by_extension(".xml").unwrap();
    assert_eq!(android.name(), "android");

    let json = Registry::by_extension(".json").unwrap();
    assert_eq!(json.name(), "jquery");
}