use clap::Parser;
use eww_hypr_config::{commands, hypr};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = commands::Cli::parse();
    match &cli.command {
        commands::Commands::Workspaces {} => loop {
            hypr::workspace_listen().await;
        },
    };
}
