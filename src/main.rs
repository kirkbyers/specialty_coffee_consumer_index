use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

mod db;
mod models;

const DB_DIR: &str = "data";
const DB_NAME: &str = "coffee_products.db";

#[tokio::main]
async fn main() -> Result<()> {
    // Create database directory if it doesn't exist
    fs::create_dir_all(DB_DIR)?;
    let db_path = Path::new(DB_DIR).join(DB_NAME);
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
    
    let examples_dir = Path::new("examples");
    let db = db::Database::new(&db_url).await?;
    
    for entry in fs::read_dir(examples_dir)? {
        let entry = entry?;
        let domain = entry.file_name().to_string_lossy().replace(".json", "");
        let url = format!("https://www.{}.com/products.json", domain);
        
        let products: models::ShopifyResponse = reqwest::Client::new()
            .get(&url)
            .send()
            .await?
            .json()
            .await?;
            
        db.save_products(&domain, &products.products).await?;
    }
    
    Ok(())
}
