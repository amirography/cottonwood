use clap::{command, Parser, Subcommand};

/// rust helper for hyprland and eww
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// show active workspace
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// creates a listener for workspaces
    Workspaces {},
}
