use std::fs::File;
use std::io::Write;
use std::time::Instant;
use clap::Parser;
use itertools::Itertools;
use log::{info, LevelFilter};
use crate::cli::{Cli, Commands, StealArgs, Websites};
use crate::config::LOGGER;
use crate::prelude::*;
use crate::products::ProductAnalyzer;

mod cakeit;
mod aniagotuje;
mod posypane;
mod cli;
pub(crate) mod model;
pub(crate) mod prelude;
pub(crate) mod config;
pub(crate) mod products;

async fn steal(arg: StealArgs) {
    let mut recipes = match arg.website {
        Websites::Aniagotuje(website_arg) => aniagotuje::steal(website_arg),
        Websites::CakeIt(website_arg) => cakeit::steal(website_arg),
        Websites::Posypane(website_arg) => posypane::steal(website_arg),
    }.await;
    info!("Successfully stole {} recipes", recipes.len());

    let prod_analyzer = ProductAnalyzer::new();
    prod_analyzer.analyze_recipes(&mut recipes);

    info!("Preparing result to display ...");
    let recipe_str: String = recipes.iter()
        .map(|r| r.as_string_line())
        .join("\n");

    if let Some(filename) = arg.filename {
        info!("Writing result to file ...");
        let mut file = File::create(filename)
            .expect("Error: Failed to create file with given filepath");
        file.write_all(recipe_str.as_bytes())
            .expect("Error: Failed to write recipes to file");
        file.flush()
            .expect("Error: Failed to flush the file");
    } else {
        println!("{recipe_str}");
    }
}

fn config() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Error: Failed to configure logger")
}

pub async fn run() {
    config();
    let cli = Cli::parse();

    let start_time = Instant::now();

    match cli.command {
        Commands::Steal(arg) => steal(arg).await
    }

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("All done in {}.{:03} seconds :)", elapsed_time.as_secs(), elapsed_time.subsec_millis());
}