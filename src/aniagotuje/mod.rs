mod api;
mod web;

use clap::Args;
use log::info;
use crate::aniagotuje::api::get_recipes;
use crate::aniagotuje::web::get_recipes_with_products;
use crate::prelude::*;
use crate::products::ProductAnalyzer;

#[derive(Args)]
pub struct AniagotujeArgs {
    /// Categories you want to steal from
    #[arg(required = true)]
    pub categories: Vec<String>
}

pub async fn steal(args: AniagotujeArgs) -> Vec<Recipe>{
    info!("Stealing from aniagotuje.pl ...");
    let categories = if args.categories.contains(&"ALL".to_string()) {
        Arc::from("")
    } else {
        Arc::from(args.categories.join(",").as_str())
    };

    let recipes_meta = get_recipes(Arc::from(categories)).await;
    let mut recipes = get_recipes_with_products(recipes_meta).await;

    let prod_analyzer = ProductAnalyzer::new();
    prod_analyzer.analyze_recipes(&mut recipes);

    recipes
}