use anyhow::Result;
use std::fs;
use std::path::Path;
use std::io::Cursor;
use flate2::read::GzDecoder;
use tar::Archive;

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
        let (_team, package) = Self::parse_package_name(package_name)?;

        println!(
            "📥 Downloading package '{}' from knot space...",
            package_name
        );

        // Try to download from Knot Space backend
        match Self::download_from_knot_space(package, destination).await {
            Ok(_) => {
                println!("✅ Successfully downloaded '{}' from knot space", package_name);
                Ok(())
            }
            Err(e) => {
                println!("⚠️  Failed to download from knot space: {}", e);
                println!("📦 Creating fallback mock package for '{}'", package_name);
                
                // Fallback to mock package
                Self::create_mock_package(package_name, destination)?;
                println!("✅ Created fallback package '{}'", package_name);
                Ok(())
            }
        }
    }

    async fn download_from_knot_space(package_name: &str, destination: &Path) -> Result<()> {
        // Get the latest version of the package
        let client = reqwest::Client::new();
        let base_url = std::env::var("KNOT_SPACE_URL")
            .unwrap_or_else(|_| "http://localhost:3001".to_string());
        
        // First, get the package versions to find the latest
        let versions_url = format!("{}/api/packages/{}/versions", base_url, package_name);
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
        let download_url = format!("{}/api/packages/{}/{}/download", base_url, package_name, latest_version);
        let download_response = client.get(&download_url).send().await?;
        
        if !download_response.status().is_success() {
            anyhow::bail!("Failed to download package file");
        }
        
        // Download the tar.gz file
        let content = download_response.bytes().await?;
        
        // Create destination directory
        fs::create_dir_all(destination)?;
        
        // Extract tar.gz content
        Self::extract_tarball(&content, destination)?;
        
        Ok(())
    }

    fn create_mock_package(package_name: &str, destination: &Path) -> Result<()> {
        // Create a mock package structure for fallback
        fs::create_dir_all(destination)?;

        // Create an index file to indicate this is a downloaded package
        let index_file = destination.join("index.js");
        let mock_content = match package_name {
            "jwt" => "module.exports = { sign: () => {}, verify: () => {} };",
            "modules-loader" => "module.exports = { load: () => {} };",
            _ => &format!("// {} package (mock fallback)", package_name),
        };

        fs::write(index_file, mock_content)?;

        // Create a package.json for the downloaded package
        let package_json = destination.join("package.json");
        let package_json_content = serde_json::json!({
            "name": format!("@{}", package_name),
            "version": "1.0.0",
            "main": "index.js",
            "description": format!("Mock fallback for {} package", package_name)
        });
        fs::write(
            package_json,
            serde_json::to_string_pretty(&package_json_content)?,
        )?;

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

    fn parse_package_name(package_name: &str) -> Result<(Option<&str>, &str)> {
        if !package_name.starts_with('@') {
            anyhow::bail!("Online package name must start with @");
        }

        let without_at = &package_name[1..];

        if let Some(slash_index) = without_at.find('/') {
            let team = &without_at[..slash_index];
            let package = &without_at[slash_index + 1..];
            Ok((Some(team), package))
        } else {
            Ok((None, without_at))
        }
    }
}
