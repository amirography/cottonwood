use clap::Parser;

mod commands;
mod hypr;
mod yuck;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = commands::Cli::parse();
    match &cli.command {
        commands::Commands::Workspaces {} => loop {
            hypr::workspace_listen().await;
        },
    };
}
