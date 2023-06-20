use log::info;
use serde::Deserialize;
use crate::prelude::*;

fn create_url_from_slug(slug: String) -> String {
    format!("https://aniagotuje.pl/przepis/{slug}")
}

fn get_url(categories_str: AStr, page: u64) -> String {
    format!("https://api.aniagotuje.pl/client/posts/search?categories={categories_str}&diets=&occasions=&tags=&page={page}&sort=publish,desc")
}

async fn get_recipes_page(categories_str: AStr, page: u64) -> Vec<Recipe> {
    reqwest::get(get_url(categories_str, page)).await.unwrap()
        .json::<ApiRes>().await.unwrap()
        .content.iter()
        .filter_map(|r| {
            if !r.recipe {
                return None
            }

            let title = r.title.clone();
            let image = r.post_thumb.url.clone();
            let link = create_url_from_slug(r.slug.clone());

            let tags: Vec<String> = r.categories.iter().filter_map(|t| {
                if t.cat_type != "TAG" {
                    return Some(t.name.clone())
                }
                None
            }).collect();

            let _products: Vec<String> = r.categories.iter().filter_map(|t| {
                if t.cat_type == "TAG" {
                    return Some(t.name.clone())
                }
                None
            }).collect();
            let products = Vec::new();

            Some(Recipe::new(title, image, link, tags, products))
        }).collect()
}


pub async fn get_recipes(categories_str: AStr) -> Vec<Recipe> {
    let pages = reqwest::get(get_url(categories_str.clone(), 0)).await.unwrap()
        .json::<ApiRes>().await.unwrap().total_pages;
    info!("Reading recipes meta data from {} pages ...", pages);
    futures::stream::iter(0..pages)
        .map(|page| get_recipes_page(categories_str.clone(), page))
        .buffered(PARALLELISM_FACTOR)
        .map(futures::stream::iter)
        .flatten()
        .collect().await
}

#[derive(Deserialize)]
struct ApiResCategory {
    name: String,
    slug: String,
    #[serde(rename(deserialize = "type"))]
    cat_type: String,
}

#[derive(Deserialize)]
struct ApiResPostThumb {
    url: String,
    #[serde(rename(deserialize = "srcSet"))]
    src_set: String,
}

#[derive(Deserialize)]
struct ApiResRecipe {
    slug: String,
    title: String,
    #[serde(rename(deserialize = "postThumb"))]
    post_thumb: ApiResPostThumb,
    categories: Vec<ApiResCategory>,
    recipe: bool

}

#[derive(Deserialize)]
struct ApiRes {
    content: Vec<ApiResRecipe>,
    #[serde(rename(deserialize = "totalPages"))]
    total_pages: u64,
}