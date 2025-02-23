use anyhow::{Context, Result};
use chrono::Local;
use glob::glob;
use specialty_coffee_consumer_index::db::Database;
use std::path::Path;
use std::fs;

const MERGED_DIR: &str = "merged_data";

#[tokio::main]
async fn main() -> Result<()> {
    // Create output directory
    fs::create_dir_all(MERGED_DIR)?;
    
    // Create merged database
    let date = Local::now().format("%Y%m%d");
    let merged_path = Path::new(MERGED_DIR)
        .join(format!("merged_coffee_products_{}.db", date));
    let merged_url = format!("sqlite:{}?mode=rwc", merged_path.display());
    
    let db = Database::new(&merged_url).await?;
    
    // Find and merge all source databases
    let pattern = "artifacts/coffee-index-db-*/coffee_products.db";
    let mut count = 0;
    
    for entry in glob(pattern)? {
        match entry {
            Ok(path) => {
                println!("Merging database: {}", path.display());
                db.merge_from(path.to_str().unwrap()).await
                    .with_context(|| format!("Failed to merge database: {}", path.display()))?;
                count += 1;
            }
            Err(e) => println!("Error processing pattern: {}", e),
        }
    }
    
    println!("Successfully merged {} databases into {}", count, merged_path.display());
    Ok(())
}