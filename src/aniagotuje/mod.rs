mod api;
mod web;

use clap::Args;
use crate::aniagotuje::api::get_recipes;
use crate::aniagotuje::web::get_recipes_with_products;
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

    let recipes_meta = get_recipes(Arc::from(categories)).await;
    let recipes = get_recipes_with_products(recipes_meta).await;
    println!("Stole {} recipes", recipes.len());
    println!("Recipe1: {:#?}", recipes.get(0).unwrap());
    println!("Recipe2: {:#?}", recipes.get(1).unwrap());
    println!("Recipe3: {:#?}", recipes.get(2).unwrap());

    recipes
}