mod api;
mod web;

use clap::Args;
use crate::aniagotuje::api::get_recipes;
use crate::prelude::*;

#[derive(Args)]
pub struct AniagotujeArgs {
    /// Categories you want to steal from
    #[arg(required = true)]
    pub categories: Vec<String>
}

pub async fn steal(args: AniagotujeArgs) -> Vec<Recipe>{
    let categories = if args.categories.contains(&"ALL".to_string()) {
        Arc::from("")
    } else { Arc::from(args.categories.join(",").as_str()) };

    let recipes = get_recipes(Arc::from(categories)).await;
    println!("Stole {} recipes", recipes.len());
    // println!("recipes: {:?}", recipes);
    todo!()
}