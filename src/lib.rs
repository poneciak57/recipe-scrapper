use clap::Parser;
use itertools::Itertools;
use log::{info, LevelFilter};
use crate::cli::{Cli, Commands};
use crate::cli::Websites::Aniagotuje;
use crate::config::LOGGER;

mod aniagotuje;
mod cli;
pub(crate) mod model;
pub(crate) mod prelude;
pub(crate) mod config;
pub(crate) mod products;

fn config() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Failed to configure logger")
}

pub async fn run() {
    config();
    let cli = Cli::parse();

    match cli.command {
        Commands::Steal(arg) => {
            let recipes = match arg.website {
                Aniagotuje(website_arg) => aniagotuje::steal(website_arg)
            }.await;
            info!("Successfully stole {} recipes", recipes.len());
            info!("Recipe1: {:#?}", recipes.get(0).unwrap());
            info!("Recipe2: {:#?}", recipes.get(1).unwrap());

            let recipe_str: Vec<String> = recipes.iter()
                .map(|r| r.into_string_line())
                .collect();

            if let Some(filename) = arg.filename {
                // write recipes to file
            }
        }
    }
}