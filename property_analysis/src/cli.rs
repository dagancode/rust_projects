use clap::Parser;

/// Property Analysis API - database seeding and server startup tool.
#[derive(Parser)]
pub struct Cli {
    #[arg(long)]
    pub seed: bool,
    #[arg(long)]
    pub force: bool,
    #[arg(long)]
    pub upsert: bool,
}
