use anyhow::Context;
use clap::Parser;
use twinex_cli::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let args = twinex_cli::cli::CliArgs::from(cli);
    twinex_cli::runner::run(args).with_context(|| "Application failed")?;
    Ok(())
}
