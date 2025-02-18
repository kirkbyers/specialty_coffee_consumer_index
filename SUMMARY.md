# Specialty Coffee Consumer Index

## Purpose
A Rust-based tool to track and analyze specialty coffee products from various roasters. The project aims to collect and store coffee product data to enable analysis of pricing, availability, and trends in the specialty coffee market.

## Current Functionality
- Reads example JSON files for coffee shop domains
- Makes HTTP requests to Shopify product APIs
- Stores product data in SQLite database
- Handles error cases and logging
- Supports rate limiting and browser-like requests

## Architecture
### Core Components
1. **Main Application**
   - Orchestrates the data collection process
   - Manages file system operations
   - Handles HTTP requests with proper headers

2. **Database Layer**
   - SQLite backend using SQLx
   - Handles product data persistence
   - Located in `db.rs`

3. **Models**
   - Data structures for Shopify API responses
   - Product and variant representations
   - Located in `models.rs`

### CI/CD
1. **GitHub Actions**
   - Daily automated runs at midnight UTC
   - Caches dependencies for faster builds
   - Stores SQLite database as artifacts
   - Supports manual workflow triggers

## Current Progress
- [x] Basic project structure
- [x] Shopify API integration
- [x] Database setup
- [x] Error handling
- [x] Automated daily data collection
- [ ] Rate limiting
- [ ] Data analysis features
- [ ] Multiple roaster support

## Dependencies
- reqwest: HTTP client
- tokio: Async runtime
- serde: JSON serialization
- sqlx: Database operations
- anyhow: Error handling

## Future Considerations
1. Add rate limiting to respect API limits
2. Implement data analysis features
3. Add support for non-Shopify stores
4. Create a web interface for data visualization
5. Add periodic data collection