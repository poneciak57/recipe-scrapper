use itertools::Itertools;

#[derive(Debug, Default)]
pub struct Recipe {
    pub title: String,
    pub image: String,
    pub link: String,
    pub tags: Vec<String>,
    pub products: Vec<String>
}

impl Recipe {
    pub fn new(title: String, image: String, link: String, tags: Vec<String>, products: Vec<String>) -> Self{
        Self {
            title,
            image,
            link,
            tags,
            products
        }
    }

    pub fn into_neo4j_query(self) -> String {
        todo!()
    }

    pub fn as_string_line(&self) -> String {
        format!("{}, {}, {}, [{}], [{}]",
                self.title,
                self.image,
                self.link,
                self.products.iter().join(","),
                self.tags.iter().join(","))
    }
}