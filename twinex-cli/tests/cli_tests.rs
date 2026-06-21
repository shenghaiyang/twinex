use clap::Parser;
use twinex_cli::cli::{CliArgs, Command};

#[test]
fn test_parse_generate_localization_file() {
    let args = CliArgs::try_parse_from(vec![
        "twinex",
        "generate",
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

    let Command::Generate {
        twine_file,
        output_path,
        gen,
        languages,
        ..
    } = args.command
    else {
        panic!("Expected Generate command");
    };

    assert_eq!(twine_file, "twine.txt");
    assert_eq!(output_path, "output.xml");
    assert_eq!(gen.format.unwrap(), "android");
    assert_eq!(languages, vec!["ko"]);
}

#[test]
fn test_validate_twine_file() {
    let args =
        CliArgs::try_parse_from(vec!["twinex", "validate", "twine.txt", "--pedantic"]).unwrap();

    let Command::Validate {
        twine_file,
        pedantic,
        ..
    } = args.command
    else {
        panic!("Expected Validate command");
    };

    assert_eq!(twine_file, "twine.txt");
    assert!(pedantic);
}

#[test]
fn test_parse_consume_localization_file() {
    let args = CliArgs::try_parse_from(vec![
        "twinex",
        "consume",
        "twine.txt",
        "ja.strings",
        "-c",
        "-m",
        "-o",
        "twine_out.txt",
    ])
    .unwrap();

    let Command::Consume {
        twine_file,
        input_path,
        con,
        ..
    } = args.command
    else {
        panic!("Expected Consume command");
    };

    assert_eq!(twine_file, "twine.txt");
    assert_eq!(input_path, "ja.strings");
    assert!(con.consume_all);
    assert!(con.consume_comments);
    assert_eq!(con.output_path.unwrap(), "twine_out.txt");
}
