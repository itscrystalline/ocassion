use clap::Parser;
use colored::Colorize;
use occasion::{config::Config, errors::ConfigError};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    /// Prints any error messages instead of failing silently.
    check: bool,
}

fn main() -> Result<(), ConfigError> {
    let flags = Cli::parse();

    let config = match Config::load_or_default(flags.check) {
        Ok(config) => config,
        Err(e) if flags.check => {
            eprintln!("{}", format!("{e}").red());
            return Err(e);
        }
        _ => return Ok(()),
    };
    println!("{}", occasion::output_of(&config));
    Ok(())
}
