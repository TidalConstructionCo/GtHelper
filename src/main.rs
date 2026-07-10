use clap::{Parser, Subcommand};
mod gtc_api;
mod listings;
mod production;

/// Helper tool to query the Galactic Tycoons API. Requires the environment variable GT_API_KEY to contain your API key.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Print all exchange listings for items you are selling.
    Listings,
    /// Print your production surplus available for contracts.
    Production,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Listings) => {
            listings::print_listings().await;
        }
        Some(Commands::Production) => {
            production::print_production().await;
        }
        None => {
            println!(
                "Running gtc without a command is not supported. Run 'gtc help' or 'gtc --help' to see available commands."
            )
        }
    }
}
