use twinex_cli::placeholders;

#[test]
fn test_convert_twine_string_placeholder() {
    assert_eq!(
        placeholders::convert_twine_string_placeholder("Hello %@"),
        "Hello %s"
    );
    assert_eq!(
        placeholders::convert_twine_string_placeholder("Value: %d, Name: %@"),
        "Value: %d, Name: %s"
    );
}

#[test]
fn test_convert_android_to_twine() {
    assert_eq!(
        placeholders::convert_placeholders_from_android_to_twine("Hello %s"),
        "Hello %@"
    );
    assert_eq!(
        placeholders::convert_placeholders_from_android_to_twine("Value: %1$s"),
        "Value: %1$@"
    );
}

#[test]
fn test_convert_flash_to_twine() {
    assert_eq!(
        placeholders::convert_placeholders_from_flash_to_twine("Hello {0}, you have {1} messages"),
        "Hello %@, you have %@ messages"
    );
}

#[test]
fn test_convert_twine_to_flash() {
    let result =
        placeholders::convert_placeholders_from_twine_to_flash("Hello %@, you have %d messages");
    assert!(result.contains("{0}"));
    assert!(result.contains("{1}"));
}

#[test]
fn test_python_specific_placeholder() {
    assert!(placeholders::contains_python_specific_placeholder(
        "Hello %(name)s"
    ));
    assert!(!placeholders::contains_python_specific_placeholder(
        "Hello %s"
    ));
    assert!(!placeholders::contains_python_specific_placeholder(
        "Hello %@"
    ));
}

#[test]
fn test_number_of_placeholders() {
    assert_eq!(placeholders::number_of_twine_placeholders("%d items"), 1);
    assert_eq!(
        placeholders::number_of_twine_placeholders("No placeholders"),
        0
    );
    assert_eq!(
        placeholders::number_of_twine_placeholders("%d items, %s name"),
        2
    );
    assert_eq!(placeholders::number_of_twine_placeholders("Hello %@"), 0);
}
