mod fs;
use fs::Config;

use clap::{command, Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "stox")]
#[command(about = "A stock market tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Test { input: String },
}

#[tokio::main]
async fn main() {
    let config: Config = Config::load();
    let cli = Cli::parse();

    match cli.command {
        Commands::Test { input: _ } => {
            println!("API key: {}", config.api_key);
        }
    }
}
