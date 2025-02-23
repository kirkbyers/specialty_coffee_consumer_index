use anyhow::{Context, Result};
use octocrab::{params::actions::ArchiveFormat, Octocrab};
use std::{env, fs, path::Path};
use zip::ZipArchive;

const ARTIFACTS_DIR: &str = "artifacts";
const OWNER: &str = "kirkbyers";
const REPO: &str = "specialty_coffee_consumer_index"; // Change this if different
const WORKFLOW_NAME: &str = "daily-coffee-index.yml";

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("GITHUB_TOKEN").context("GITHUB_TOKEN not set")?;
    let num_runs = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(5); // Default to last 5 runs

    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()?;

    // Create artifacts directory
    fs::create_dir_all(ARTIFACTS_DIR)?;

    // Get workflow runs
    let runs = octocrab
        .workflows(OWNER, REPO)
        .list_runs(WORKFLOW_NAME)
        .page(1u32)
        .per_page(num_runs)
        .send()
        .await?;

    println!("Fetching artifacts from the last {} runs", num_runs);

    for run in runs.items {
        println!("Processing run #{}", run.run_number);
        
        let artifacts = octocrab
            .actions()
            .list_workflow_run_artifacts(OWNER, REPO, run.id)
            .send()
            .await?;

        for artifact in artifacts.value.unwrap_or_default().items {
            if artifact.name.starts_with("coffee-index-db-") {
                // Download the zip file
                let zip_content = octocrab
                    .actions()
                    .download_artifact(OWNER, REPO, artifact.id, ArchiveFormat::Zip)
                    .await?;

                
                // Create run-specific directory
                let run_dir = Path::new(ARTIFACTS_DIR)
                    .join(format!("coffee-index-db-{}", run.run_number));
                fs::create_dir_all(&run_dir)?;

                // Save and extract zip
                let zip_path = run_dir.join("artifact.zip");
                fs::write(&zip_path, &zip_content)?;

                let zip_file = fs::File::open(&zip_path)?;
                let mut archive = ZipArchive::new(zip_file)?;
                archive.extract(&run_dir)?;

                // Clean up zip file
                fs::remove_file(zip_path)?;

                println!("Downloaded artifacts for run #{}", run.run_number);
            }
        }
    }

    Ok(())
}