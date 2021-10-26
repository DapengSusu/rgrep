use clap::Parser;
use rgrep::*;

fn main() -> anyhow::Result<()> {
    let config = GrepConfig::parse();

    config.match_with_default_strategy()?;

    Ok(())
}
