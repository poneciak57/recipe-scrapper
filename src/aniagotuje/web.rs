use itertools::Itertools;
use log::info;
use scraper::{Html, Selector};
use crate::prelude::*;
use crate::products::ProductState;

async fn get_products_for_recipe(mut recipe: Recipe) -> Recipe {
    let ingredients_list: Selector = Selector::parse(".recipe-ing-list").unwrap();
    let ingredient: Selector = Selector::parse("li").unwrap();
    let res = reqwest::get(&recipe.link).await.unwrap()
        .text().await.unwrap();
    let document = Html::parse_document(&res);
    let list_of_products: Vec<ProductState> = document.select(&ingredients_list)
        .map(|l| l.select(&ingredient)
            .map(|i| ProductState::WAITING(i.text().join(""))).collect::<Vec<ProductState>>()
        )
        .flatten()
        .collect();
    recipe.products = list_of_products;
    recipe
}

pub async fn get_recipes_with_products(recipes: Vec<Recipe>) -> Vec<Recipe> {
    info!("Reading products for {} recipes ...", recipes.len());
    futures::stream::iter(recipes)
        .map(|r| get_products_for_recipe(r))
        .buffered(PARALLELISM_FACTOR)
        .collect()
        .await
}