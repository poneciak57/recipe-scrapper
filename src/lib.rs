use clap::Parser;
use crate::cli::{Cli, Commands};
use crate::cli::Websites::Aniagotuje;

mod aniagotuje;
mod cli;
pub(crate) mod model;
pub(crate) mod prelude;
pub(crate) mod config;

fn config() {

}

pub async fn run() {
    config();
    let cli = Cli::parse();

    match cli.command {
        Commands::Steal(arg) => {
            let recipes = match arg.website {
                Aniagotuje(website_arg) => aniagotuje::steal(website_arg)
            }.await;
            if let Some(filename) = arg.filename {
                // write recipes to file
            }
        }
    }
}