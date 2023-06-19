use clap::Parser;
use crate::config::{Cli, Commands};
use crate::config::Websites::Aniagotuje;

mod aniagotuje;
mod config;


pub async fn run() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Steal(arg) => match arg.website {
            Aniagotuje(website_arg) => aniagotuje::steal(website_arg, arg.filename)
        }
    }.await
}