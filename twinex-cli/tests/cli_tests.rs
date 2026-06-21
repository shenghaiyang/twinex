use clap::Parser;
use twinex_cli::cli::Cli;

#[test]
fn test_parse_generate_localization_file() {
    let cli = Cli::try_parse_from(vec![
        "twinex",
        "generate-localization-file",
        "twine.txt",
        "output.xml",
        "--tags",
        "FT,FB",
        "--format",
        "android",
        "--lang",
        "ko",
    ])
    .unwrap();

    let args = twinex_cli::cli::CliArgs::from(cli);
    assert_eq!(args.command, "generate-localization-file");
    assert_eq!(args.twine_file, "twine.txt");
    assert_eq!(args.output_path.unwrap(), "output.xml");
    assert_eq!(args.format.unwrap(), "android");
    assert_eq!(args.languages, vec!["ko"]);
}

#[test]
fn test_validate_twine_file() {
    let cli = Cli::try_parse_from(vec![
        "twinex",
        "validate-twine-file",
        "twine.txt",
        "--pedantic",
    ])
    .unwrap();

    let args = twinex_cli::cli::CliArgs::from(cli);
    assert_eq!(args.command, "validate-twine-file");
    assert_eq!(args.twine_file, "twine.txt");
    assert!(args.pedantic);
}

#[test]
fn test_parse_consume_localization_file() {
    let cli = Cli::try_parse_from(vec![
        "twinex",
        "consume-localization-file",
        "twine.txt",
        "ja.strings",
        "-a",
        "-c",
        "-o",
        "twine_out.txt",
    ])
    .unwrap();

    let args = twinex_cli::cli::CliArgs::from(cli);
    assert_eq!(args.command, "consume-localization-file");
    assert_eq!(args.twine_file, "twine.txt");
    assert_eq!(args.input_path.unwrap(), "ja.strings");
    assert!(args.consume_all);
    assert!(args.consume_comments);
    assert_eq!(args.output_path.unwrap(), "twine_out.txt");
}
