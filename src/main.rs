mod api;
mod fs;
mod types;

use types::{Config, Save};

use clap::{command, Parser, Subcommand};

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
    #[command(about = "Current stock price for a given ticker")]
    Current { ticker: String },

    #[command(arg_required_else_help = true)]
    #[command(about = "Currency conversion")]
    Convert {
        from: String,
        to: String,
        amount: f64,
    },
}

#[tokio::main]
async fn main() {
    let config: Config = Config::load();

    let save: Save = Save::load();

    let cli = Cli::parse();

    match cli.command {
        Commands::Current { ticker } => {
            let price = api::get_price(&config, ticker);
            println!("Current price: ${}", price.await);
        }
        Commands::Convert { from, to, amount } => {
            let amount = api::get_conversion(&config, from, to, amount);
            println!("Conversion: {}", amount.await);
        }
    }
}
