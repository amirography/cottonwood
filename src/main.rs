#[warn(clippy::pedantic)]
use clap::Parser;
use cottonwood::{commands, hypr};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = commands::Cli::parse();
    match &cli.command {
        commands::Commands::Workspaces {} => loop {
            hypr::workspace_listen().await;
        },
    };
}
