use clap::{Args, Parser, Subcommand};
use crate::aniagotuje::AniagotujeArgs;

/// Program for stealing recipes from popular websites
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum  Commands {
    /// Steals recipes from given website
    Steal(StealArgs),
}

#[derive(Subcommand)]
pub enum Websites {
    /// Steal from aniagotuje.pl
    Aniagotuje (AniagotujeArgs)
}

#[derive(Args)]
pub struct StealArgs {
    /// Website you want to steal from
    #[command(subcommand)]
    pub website: Websites,

    /// If you want to save result to file
    #[arg(short, long)]
    pub filename: Option<String>
}