use clap::{Args, Parser, Subcommand};

/// Command line tool for managing strings and their translations.
#[derive(Parser)]
#[command(name = "twinex", version, about, arg_required_else_help = true)]
pub struct CliArgs {
    #[command(subcommand)]
    #[allow(private_interfaces)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Generate localization file(s) from a twine file
    Generate {
        /// The twine data file
        twine_file: String,
        /// The output path (file, directory, or archive)
        output_path: String,
        #[command(flatten)]
        gen: GenerateArgs,
        /// Generate all localization files (output_path is a directory)
        #[arg(short = 'a', long)]
        all: bool,
        /// Generate a zip archive of localization files
        #[arg(long)]
        archive: bool,
        #[arg(short = 'l', long = "lang", value_delimiter = ',')]
        languages: Vec<String>,
        #[arg(short = 'r', long = "create-folders")]
        create_folders: bool,
        #[arg(short = 'n', long = "file-name")]
        file_name: Option<String>,
        #[arg(long)]
        validate: bool,
    },
    /// Consume translations from localization file(s) into the twine file
    Consume {
        /// The twine data file
        twine_file: String,
        /// The input path (file, directory, or archive)
        input_path: String,
        #[command(flatten)]
        con: ConsumeArgs,
        /// Consume all localization files from a directory
        #[arg(short = 'a', long)]
        all: bool,
        #[arg(short = 'l', long = "lang", value_delimiter = ',')]
        languages: Vec<String>,
    },
    /// Validate that a twine file is parseable
    Validate {
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
pub struct GenerateArgs {
    #[arg(short = 'd', long = "developer-language")]
    pub developer_language: Option<String>,
    #[arg(short = 'e', long)]
    pub encoding: Option<String>,
    #[arg(long = "escape-all-tags")]
    pub escape_all_tags: bool,
    #[arg(short = 'f', long)]
    pub format: Option<String>,
    #[arg(short = 'i', long, default_value = "all")]
    pub include: String,
    #[arg(short = 'q', long)]
    pub quiet: bool,
    #[arg(short = 't', long, value_delimiter = ',')]
    pub tags: Vec<String>,
    #[arg(short = 'u', long)]
    pub untagged: bool,
}

#[derive(Args)]
pub struct ConsumeArgs {
    #[arg(short = 'c', long = "consume-all")]
    pub consume_all: bool,
    #[arg(short = 'm', long = "consume-comments")]
    pub consume_comments: bool,
    #[arg(short = 'd', long = "developer-language")]
    pub developer_language: Option<String>,
    #[arg(short = 'e', long)]
    pub encoding: Option<String>,
    #[arg(short = 'f', long)]
    pub format: Option<String>,
    #[arg(short = 'o', long = "output-file")]
    pub output_path: Option<String>,
    #[arg(short = 'q', long)]
    pub quiet: bool,
    #[arg(short = 't', long, value_delimiter = ',')]
    pub tags: Vec<String>,
}
