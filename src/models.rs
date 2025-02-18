use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShopifyResponse {
    pub products: Vec<Product>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub title: String,
    pub handle: String,
    pub published_at: String,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variant {
    pub id: i64,
    pub title: String,
    pub price: String,
    pub available: bool,
}
