use regex::Regex;
use std::sync::OnceLock;

const PLACEHOLDER_FLAGS: &str =
    r"([-+0#])?(\d+|\*)?(\.(\d+|\*))?(hh?|ll?|L|z|j|t|q)?";
const PLACEHOLDER_PARAMETER_FLAGS_WIDTH_PRECISION_LENGTH: &str =
    r"(\d+\$)?([-+0#])?(\d+|\*)?(\.(\d+|\*))?(hh?|ll?|L|z|j|t|q)?";
const PLACEHOLDER_TYPES: &str = r"[diufFeEgGxXoscpaA]";

fn placeholder_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        let pattern = format!(
            r"%{}{}",
            PLACEHOLDER_PARAMETER_FLAGS_WIDTH_PRECISION_LENGTH, PLACEHOLDER_TYPES
        );
        Regex::new(&pattern).unwrap()
    })
}

fn twine_string_placeholder_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        let pattern = format!(
            r"(%{})@",
            PLACEHOLDER_PARAMETER_FLAGS_WIDTH_PRECISION_LENGTH
        );
        Regex::new(&pattern).unwrap()
    })
}

pub fn number_of_twine_placeholders(input: &str) -> usize {
    placeholder_regex().find_iter(input).count()
}

pub fn convert_twine_string_placeholder(input: &str) -> String {
    // %@ -> %s
    twine_string_placeholder_regex()
        .replace_all(input, "${1}s")
        .to_string()
}

pub fn convert_placeholders_from_twine_to_android(input: &str) -> String {
    let value = convert_twine_string_placeholder(input);
    let num_placeholders = number_of_twine_placeholders(&value);

    if num_placeholders == 0 {
        return value;
    }

    // Double single percent signs
    let single_percent_regex = Regex::new(
        &format!(
            r"([^%])(%)(?!(%|{}{}))",
            PLACEHOLDER_PARAMETER_FLAGS_WIDTH_PRECISION_LENGTH, PLACEHOLDER_TYPES
        )
    )
    .unwrap();
    let value = single_percent_regex.replace_all(&value, "${1}%%").to_string();

    if num_placeholders < 2 {
        return value;
    }

    // Number non-numbered placeholders
    let non_numbered_regex = Regex::new(&format!(
        r"%({}{})",
        PLACEHOLDER_FLAGS, PLACEHOLDER_TYPES
    ))
    .unwrap();

    let non_numbered_count = non_numbered_regex.find_iter(&value).count();
    if non_numbered_count == 0 {
        return value;
    }

    let mut index = 0usize;
    let result = non_numbered_regex
        .replace_all(&value, |caps: &regex::Captures| {
            index += 1;
            let captured = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            format!("%{index}${captured}")
        })
        .to_string();

    result
}

pub fn convert_placeholders_from_android_to_twine(input: &str) -> String {
    let pattern = format!(
        r"(%{})s",
        PLACEHOLDER_PARAMETER_FLAGS_WIDTH_PRECISION_LENGTH
    );
    let re = Regex::new(&pattern).unwrap();
    re.replace_all(input, "${1}@").to_string()
}

pub fn convert_placeholders_from_twine_to_flash(input: &str) -> String {
    let value = convert_twine_string_placeholder(input);
    let mut index = 0usize;
    placeholder_regex()
        .replace_all(&value, |_: &regex::Captures| {
            let result = format!("{{{}}}", index);
            index += 1;
            result
        })
        .to_string()
}

pub fn convert_placeholders_from_flash_to_twine(input: &str) -> String {
    let re = Regex::new(r"\{\d+\}").unwrap();
    re.replace_all(input, "%@").to_string()
}

pub fn contains_python_specific_placeholder(input: &str) -> bool {
    let pattern = format!(
        r"%\([a-zA-Z0-9_-]+\){}{}",
        PLACEHOLDER_PARAMETER_FLAGS_WIDTH_PRECISION_LENGTH, PLACEHOLDER_TYPES
    );
    let re = Regex::new(&pattern).unwrap();
    re.is_match(input)
}