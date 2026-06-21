use anyhow::Context;
use clap::Parser;
use twinex_cli::cli::CliArgs;

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    twinex_cli::runner::run(args).with_context(|| "Application failed")?;
    Ok(())
}
