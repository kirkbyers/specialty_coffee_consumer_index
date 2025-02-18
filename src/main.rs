use anyhow::{Context, Result};
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
    
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;
    
    for entry in fs::read_dir(examples_dir)? {
        let entry = entry?;
        let domain = entry.file_name().to_string_lossy().replace(".json", "");
        let url = format!("https://www.{}.com/products.json", domain);
        
        println!("Fetching products from: {}", url);
        let response = client.get(&url)
            .header("Accept", "application/json")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("Connection", "keep-alive")
            .header("Cache-Control", "no-cache")
            .send()
            .await
            .with_context(|| format!("Failed to fetch from {}", url))?;
            
        let status = response.status();
        println!("Response status: {}", status);
        
        if !status.is_success() {
            println!("Response text: {}", response.text().await?);
            continue;
        }
        
        let products: models::ShopifyResponse = response.json()
            .await
            .with_context(|| format!("Failed to parse JSON from {}", url))?;
            
        db.save_products(&domain, &products.products).await?;
    }
    
    Ok(())
}
