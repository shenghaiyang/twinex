use clap::{Args, Parser, Subcommand};

/// Command line tool for managing strings and their translations.
#[derive(Parser)]
#[command(name = "twinex", version, about, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    #[allow(private_interfaces)]
    pub command: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    /// Generate a single localization file
    #[command(name = "generate-localization-file")]
    GenerateLocalizationFile {
        /// The twine data file
        twine_file: String,
        /// The output localization file path
        output_path: String,
        #[command(flatten)]
        gen: GenerateArgs,
        #[arg(short = 'l', long = "lang", value_delimiter = ',')]
        languages: Vec<String>,
        #[arg(long)]
        validate: bool,
    },
    /// Generate all localization files for a project
    #[command(name = "generate-all-localization-files")]
    GenerateAllLocalizationFiles {
        /// The twine data file
        twine_file: String,
        /// The output directory path
        output_path: String,
        #[command(flatten)]
        gen: GenerateArgs,
        #[arg(short = 'r', long = "create-folders")]
        create_folders: bool,
        #[arg(short = 'n', long = "file-name")]
        file_name: Option<String>,
        #[arg(long)]
        validate: bool,
    },
    /// Generate a zip archive of localization files
    #[command(name = "generate-localization-archive")]
    GenerateLocalizationArchive {
        /// The twine data file
        twine_file: String,
        /// The output archive path
        output_path: String,
        #[command(flatten)]
        gen: GenerateArgs,
        #[arg(short = 'f', long)]
        format: String,
        #[arg(short = 'l', long = "lang", value_delimiter = ',')]
        languages: Vec<String>,
        #[arg(long)]
        validate: bool,
    },
    /// Consume translations from a localization file into the twine file
    #[command(name = "consume-localization-file")]
    ConsumeLocalizationFile {
        /// The twine data file
        twine_file: String,
        /// The input localization file path
        input_path: String,
        #[command(flatten)]
        con: ConsumeArgs,
        #[arg(short = 'l', long = "lang", value_delimiter = ',')]
        languages: Vec<String>,
    },
    /// Consume translations from a directory into the twine file
    #[command(name = "consume-all-localization-files")]
    ConsumeAllLocalizationFiles {
        /// The twine data file
        twine_file: String,
        /// The input directory path
        input_path: String,
        #[command(flatten)]
        con: ConsumeArgs,
    },
    /// Consume translations from an archive into the twine file
    #[command(name = "consume-localization-archive")]
    ConsumeLocalizationArchive {
        /// The twine data file
        twine_file: String,
        /// The input archive path
        input_path: String,
        #[command(flatten)]
        con: ConsumeArgs,
    },
    /// Validate that a Twine file is parseable
    #[command(name = "validate-twine-file")]
    ValidateTwineFile {
        /// The twine data file
        twine_file: String,
        #[arg(short = 'd', long)]
        developer_language: Option<String>,
        #[arg(short = 'p', long)]
        pedantic: bool,
        #[arg(short = 'q', long)]
        quiet: bool,
    },
}

#[derive(Args)]
struct GenerateArgs {
    #[arg(short = 'd', long = "developer-language")]
    developer_language: Option<String>,
    #[arg(short = 'e', long)]
    encoding: Option<String>,
    #[arg(long = "escape-all-tags")]
    escape_all_tags: bool,
    #[arg(short = 'f', long)]
    format: Option<String>,
    #[arg(short = 'i', long, default_value = "all")]
    include: String,
    #[arg(short = 'q', long)]
    quiet: bool,
    #[arg(short = 't', long, value_delimiter = ',')]
    tags: Vec<String>,
    #[arg(short = 'u', long)]
    untagged: bool,
}

#[derive(Args)]
struct ConsumeArgs {
    #[arg(short = 'a', long = "consume-all")]
    consume_all: bool,
    #[arg(short = 'c', long = "consume-comments")]
    consume_comments: bool,
    #[arg(short = 'd', long = "developer-language")]
    developer_language: Option<String>,
    #[arg(short = 'e', long)]
    encoding: Option<String>,
    #[arg(short = 'f', long)]
    format: Option<String>,
    #[arg(short = 'o', long = "output-file")]
    output_path: Option<String>,
    #[arg(short = 'q', long)]
    quiet: bool,
    #[arg(short = 't', long, value_delimiter = ',')]
    tags: Vec<String>,
}

// ── Flat args struct consumed by runner ───────────────────────

/// Flat representation of all CLI arguments, consumed by the runner.
pub struct CliArgs {
    pub command: String,
    pub twine_file: String,
    pub input_path: Option<String>,
    pub output_path: Option<String>,
    pub consume_all: bool,
    pub consume_comments: bool,
    pub create_folders: bool,
    pub developer_language: Option<String>,
    pub encoding: Option<String>,
    pub escape_all_tags: bool,
    pub file_name: Option<String>,
    pub format: Option<String>,
    pub include: String,
    pub languages: Vec<String>,
    pub pedantic: bool,
    pub quiet: bool,
    pub tags: Vec<Vec<String>>,
    pub untagged: bool,
    pub validate: bool,
}

impl From<Cli> for CliArgs {
    fn from(cli: Cli) -> Self {
        match cli.command {
            SubCommand::GenerateLocalizationFile {
                twine_file,
                output_path,
                gen,
                languages,
                validate,
            } => CliArgs {
                command: "generate-localization-file".into(),
                twine_file,
                output_path: Some(output_path),
                developer_language: gen.developer_language,
                encoding: gen.encoding,
                escape_all_tags: gen.escape_all_tags,
                format: gen.format,
                include: gen.include,
                languages,
                quiet: gen.quiet,
                tags: wrap_tags(gen.tags),
                untagged: gen.untagged,
                validate,
                ..Default::default()
            },
            SubCommand::GenerateAllLocalizationFiles {
                twine_file,
                output_path,
                gen,
                create_folders,
                file_name,
                validate,
            } => CliArgs {
                command: "generate-all-localization-files".into(),
                twine_file,
                output_path: Some(output_path),
                create_folders,
                file_name,
                developer_language: gen.developer_language,
                encoding: gen.encoding,
                escape_all_tags: gen.escape_all_tags,
                format: gen.format,
                include: gen.include,
                quiet: gen.quiet,
                tags: wrap_tags(gen.tags),
                untagged: gen.untagged,
                validate,
                ..Default::default()
            },
            SubCommand::GenerateLocalizationArchive {
                twine_file,
                output_path,
                gen,
                format,
                languages,
                validate,
            } => CliArgs {
                command: "generate-localization-archive".into(),
                twine_file,
                output_path: Some(output_path),
                format: Some(format),
                developer_language: gen.developer_language,
                encoding: gen.encoding,
                escape_all_tags: gen.escape_all_tags,
                include: gen.include,
                languages,
                quiet: gen.quiet,
                tags: wrap_tags(gen.tags),
                untagged: gen.untagged,
                validate,
                ..Default::default()
            },
            SubCommand::ConsumeLocalizationFile {
                twine_file,
                input_path,
                con,
                languages,
            } => CliArgs {
                command: "consume-localization-file".into(),
                twine_file,
                input_path: Some(input_path),
                languages,
                consume_all: con.consume_all,
                consume_comments: con.consume_comments,
                developer_language: con.developer_language,
                encoding: con.encoding,
                format: con.format,
                output_path: con.output_path,
                quiet: con.quiet,
                tags: wrap_tags(con.tags),
                ..Default::default()
            },
            SubCommand::ConsumeAllLocalizationFiles {
                twine_file,
                input_path,
                con,
            } => CliArgs {
                command: "consume-all-localization-files".into(),
                twine_file,
                input_path: Some(input_path),
                consume_all: con.consume_all,
                consume_comments: con.consume_comments,
                developer_language: con.developer_language,
                encoding: con.encoding,
                format: con.format,
                output_path: con.output_path,
                quiet: con.quiet,
                tags: wrap_tags(con.tags),
                ..Default::default()
            },
            SubCommand::ConsumeLocalizationArchive {
                twine_file,
                input_path,
                con,
            } => CliArgs {
                command: "consume-localization-archive".into(),
                twine_file,
                input_path: Some(input_path),
                consume_all: con.consume_all,
                consume_comments: con.consume_comments,
                developer_language: con.developer_language,
                encoding: con.encoding,
                format: con.format,
                output_path: con.output_path,
                quiet: con.quiet,
                tags: wrap_tags(con.tags),
                ..Default::default()
            },
            SubCommand::ValidateTwineFile {
                twine_file,
                developer_language,
                pedantic,
                quiet,
            } => CliArgs {
                command: "validate-twine-file".into(),
                twine_file,
                developer_language,
                pedantic,
                quiet,
                ..Default::default()
            },
        }
    }
}

impl Default for CliArgs {
    fn default() -> Self {
        CliArgs {
            command: String::new(),
            twine_file: String::new(),
            input_path: None,
            output_path: None,
            consume_all: false,
            consume_comments: false,
            create_folders: false,
            developer_language: None,
            encoding: None,
            escape_all_tags: false,
            file_name: None,
            format: None,
            include: "all".into(),
            languages: Vec::new(),
            pedantic: false,
            quiet: false,
            tags: Vec::new(),
            untagged: false,
            validate: false,
        }
    }
}

fn wrap_tags(tags: Vec<String>) -> Vec<Vec<String>> {
    if tags.is_empty() {
        Vec::new()
    } else {
        vec![tags]
    }
}