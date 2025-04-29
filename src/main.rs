use clap::Parser;
use colored::Colorize;
use occasion::config::Config;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    /// Prints any error messages instead of failing silently.
    check: bool,
}

fn main() {
    let flags = Cli::parse();

    let config = match Config::load_or_default() {
        Ok(config) => config,
        Err(e) if flags.check => {
            eprintln!("{}", format!("{e}").red());
            return;
        }
        _ => return,
    };
    println!("{}", occasion::output_of(&config));
}
