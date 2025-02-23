use anyhow::Result;
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use crate::models::Product;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(db_path: &str) -> Result<Self> {
        let pool = SqlitePool::connect(&format!("sqlite:{}", db_path)).await?;
        
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS products (
                id INTEGER PRIMARY KEY,
                domain TEXT NOT NULL,
                product_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                handle TEXT NOT NULL,
                published_at TEXT NOT NULL,
                collected_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS variants (
                id INTEGER PRIMARY KEY,
                product_id INTEGER NOT NULL,
                variant_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                price TEXT NOT NULL,
                available BOOLEAN NOT NULL,
                FOREIGN KEY(product_id) REFERENCES products(id)
            )"
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn save_products(&self, domain: &str, products: &[Product]) -> Result<()> {
        for product in products {
            let product_id = sqlx::query(
                "INSERT INTO products (domain, product_id, title, handle, published_at) 
                 VALUES (?, ?, ?, ?, ?) RETURNING id"
            )
            .bind(domain)
            .bind(product.id)
            .bind(&product.title)
            .bind(&product.handle)
            .bind(&product.published_at)
            .fetch_one(&self.pool)
            .await?
            .get::<i64, _>(0);

            for variant in &product.variants {
                sqlx::query(
                    "INSERT INTO variants (product_id, variant_id, title, price, available)
                     VALUES (?, ?, ?, ?, ?)"
                )
                .bind(product_id)
                .bind(variant.id)
                .bind(&variant.title)
                .bind(&variant.price)
                .bind(variant.available)
                .execute(&self.pool)
                .await?;
            }
        }
        Ok(())
    }

    pub async fn merge_from(&self, source_path: &str) -> Result<()> {
        let source_pool = SqlitePool::connect(&format!("sqlite:{}", source_path)).await?;

        // Transfer products
        let products = sqlx::query(
            "SELECT domain, product_id, title, handle, published_at FROM products"
        )
        .fetch_all(&source_pool)
        .await?;

        for product in products {
            let product_id = sqlx::query(
                "INSERT OR IGNORE INTO products (domain, product_id, title, handle, published_at) 
                 VALUES (?, ?, ?, ?, ?) RETURNING id"
            )
            .bind(product.get::<&str, _>("domain"))
            .bind(product.get::<i64, _>("product_id"))
            .bind(product.get::<&str, _>("title"))
            .bind(product.get::<&str, _>("handle"))
            .bind(product.get::<&str, _>("published_at"))
            .fetch_one(&self.pool)
            .await?
            .get::<i64, _>(0);

            // Transfer associated variants
            let variants = sqlx::query(
                "SELECT variant_id, title, price, available FROM variants WHERE product_id = ?"
            )
            .bind(product_id)
            .fetch_all(&source_pool)
            .await?;

            for variant in variants {
                sqlx::query(
                    "INSERT OR IGNORE INTO variants (product_id, variant_id, title, price, available)
                     VALUES (?, ?, ?, ?, ?)"
                )
                .bind(product_id)
                .bind(variant.get::<i64, _>("variant_id"))
                .bind(variant.get::<&str, _>("title"))
                .bind(variant.get::<&str, _>("price"))
                .bind(variant.get::<bool, _>("available"))
                .execute(&self.pool)
                .await?;
            }
        }

        Ok(())
    }
}
