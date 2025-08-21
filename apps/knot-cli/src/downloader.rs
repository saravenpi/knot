use anyhow::Result;
use std::fs;
use std::path::Path;
use std::io::Cursor;
use std::env;
use flate2::read::GzDecoder;
use tar::Archive;

// Helper function to get the Knot Space URL (same as in commands.rs)
fn get_knot_space_url() -> String {
    env::var("KNOT_SPACE_URL").unwrap_or_else(|_| "https://knot-space-production.up.railway.app".to_string())
}

pub struct PackageDownloader;

impl PackageDownloader {
    pub async fn download_package(package_name: &str, destination: &Path) -> Result<()> {
        if package_name.starts_with('@') {
            Self::download_online_package(package_name, destination).await
        } else {
            anyhow::bail!("Package '{}' should be a local package", package_name)
        }
    }

    async fn download_online_package(package_name: &str, destination: &Path) -> Result<()> {
        println!(
            "📥 Downloading package '{}' from knot space...",
            package_name
        );

        // Try to download from Knot Space backend
        match Self::download_from_knot_space(package_name, destination).await {
            Ok(_) => {
                println!("✅ Successfully downloaded '{}' from knot space", package_name);
                Ok(())
            }
            Err(e) => {
                eprintln!("❌ Failed to download package '{}': {}", package_name, e);
                anyhow::bail!("Package download failed");
            }
        }
    }

    async fn download_from_knot_space(package_name: &str, destination: &Path) -> Result<()> {
        // Get the latest version of the package
        let client = reqwest::Client::new();
        let base_url = get_knot_space_url();
        
        // Strip @ prefix for API calls - the API expects package names without @
        let api_package_name = if let Some(stripped) = package_name.strip_prefix('@') {
            stripped
        } else {
            package_name
        };
        
        // First, get the package versions to find the latest
        let versions_url = format!("{}/api/packages/{}/versions", base_url, api_package_name);
        let versions_response = client.get(&versions_url).send().await?;
        
        if !versions_response.status().is_success() {
            anyhow::bail!("Package '{}' not found in knot space", package_name);
        }
        
        let versions_data: serde_json::Value = versions_response.json().await?;
        let versions = versions_data["data"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid versions response format"))?;
            
        if versions.is_empty() {
            anyhow::bail!("No versions found for package '{}'", package_name);
        }
        
        // Get the latest version (first in the array, sorted by publish date desc)
        let latest_version = versions[0]["version"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid version format"))?;
        
        // Download the package file
        let download_url = format!("{}/api/packages/{}/{}/download", base_url, api_package_name, latest_version);
        let download_response = client.get(&download_url).send().await?;
        
        if !download_response.status().is_success() {
            anyhow::bail!("Failed to download package file (HTTP {})", download_response.status());
        }
        
        // Download the tar.gz file
        let content = download_response.bytes().await?;
        
        // Create destination directory
        fs::create_dir_all(destination)?;
        
        // Extract tar.gz content
        Self::extract_tarball(&content, destination)?;
        
        Ok(())
    }


    fn extract_tarball(content: &[u8], destination: &Path) -> Result<()> {
        // Create a cursor from the bytes
        let cursor = Cursor::new(content);
        
        // Decompress with gzip
        let decoder = GzDecoder::new(cursor);
        
        // Create a tar archive
        let mut archive = Archive::new(decoder);
        
        // Extract all files to the destination
        archive.unpack(destination)?;
        
        println!("📦 Extracted package contents to: {}", destination.display());
        Ok(())
    }

}
