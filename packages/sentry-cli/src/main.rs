use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Audit a contract for compliance events
    Audit {
        #[arg(short, long)]
        contract: String,
        #[arg(short, long)]
        from_ledger: Option<u32>,
    },
    /// Status of the Sentry system
    Status,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Audit { contract, from_ledger } => {
            println!("Auditing contract {} from ledger {:?}", contract, from_ledger);
            // TODO: Connect to DB and fetch AuditRecords
        }
        Commands::Status => {
            println!("Soroban Sentry: Systems Operational");
        }
    }

    Ok(())
}
