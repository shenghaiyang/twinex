use crate::cli::{CliArgs, Command};
use crate::twinex::Twinex;
use crate::twinex_params::{ConsumeParams, GenerateParams, ValidateParams};

pub fn run(args: CliArgs) -> anyhow::Result<()> {
    let twinex = Twinex;
    match args.command {
        Command::Generate {
            twine_file,
            output_path,
            gen,
            all,
            archive,
            languages,
            create_folders,
            file_name,
            validate,
        } => Ok(twinex.generate(&GenerateParams {
            twine_file,
            output_path,
            developer_language: gen.developer_language,
            encoding: gen.encoding,
            escape_all_tags: gen.escape_all_tags,
            format: gen.format,
            include: gen.include,
            quiet: gen.quiet,
            tags: gen.tags,
            untagged: gen.untagged,
            validate,
            all,
            archive,
            languages,
            create_folders,
            file_name,
        })?),
        Command::Consume {
            twine_file,
            input_path,
            con,
            all,
            languages,
        } => Ok(twinex.consume(&ConsumeParams {
            twine_file,
            input_path,
            consume_all: con.consume_all,
            consume_comments: con.consume_comments,
            developer_language: con.developer_language,
            encoding: con.encoding,
            format: con.format,
            output_path: con.output_path,
            quiet: con.quiet,
            tags: con.tags,
            all,
            languages,
        })?),
        Command::Validate {
            twine_file,
            developer_language,
            pedantic,
            quiet,
        } => Ok(twinex.validate(&ValidateParams {
            twine_file,
            developer_language,
            pedantic,
            quiet,
        })?),
    }
}
