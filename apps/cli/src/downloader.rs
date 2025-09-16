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
    pub async fn download_package(package_spec: &str, destination: &Path) -> Result<()> {
        if package_spec.starts_with('@') {
            Self::download_online_package(package_spec, destination).await
        } else {
            anyhow::bail!("Cannot download package '{}': Local packages are not downloadable\n💡 Local packages should be available in the packages/ directory\n💡 Use '@package-name' for packages from Knot Space\n💡 Example: @my-team/my-package", package_spec)
        }
    }

    fn parse_package_spec(package_spec: &str) -> (String, Option<String>) {
        if let Some(stripped) = package_spec.strip_prefix('@') {
            // Handle scoped packages like @hono-modules-loader@0.2.5
            if let Some(at_pos) = stripped.find('@') {
                // Found a second @ after the first one
                let at_pos = at_pos + 1; // Adjust for the skipped first character
                let name = package_spec[..at_pos].to_string();
                let version = package_spec[at_pos + 1..].to_string();
                (name, Some(version))
            } else {
                // No version specified, just the scoped package name
                (package_spec.to_string(), None)
            }
        } else {
            // Handle regular packages like package-name@1.0.0
            if let Some(at_pos) = package_spec.rfind('@') {
                let name = package_spec[..at_pos].to_string();
                let version = package_spec[at_pos + 1..].to_string();
                (name, Some(version))
            } else {
                (package_spec.to_string(), None)
            }
        }
    }

    async fn download_online_package(package_spec: &str, destination: &Path) -> Result<()> {
        println!(
            "📥 Downloading package '{}' from knot space...",
            package_spec
        );

        // Parse package spec to get name and version
        let (package_name, version) = Self::parse_package_spec(package_spec);

        // Try to download from Knot Space backend
        match Self::download_from_knot_space(&package_name, version.as_deref(), destination).await {
            Ok(_) => {
                println!("✅ Successfully downloaded '{}' from knot space", package_spec);
                Ok(())
            }
            Err(e) => {
                anyhow::bail!("Failed to download package '{}' from Knot Space: {}\n💡 Check your internet connection\n💡 Verify the package name and version are correct\n💡 Ensure the package exists on Knot Space", package_spec, e);
            }
        }
    }

    async fn download_from_knot_space(package_name: &str, requested_version: Option<&str>, destination: &Path) -> Result<()> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()?;
        let base_url = get_knot_space_url();
        
        // Strip @ prefix for API calls - the API expects package names without @
        let api_package_name = if let Some(stripped) = package_name.strip_prefix('@') {
            stripped
        } else {
            package_name
        };
        
        // Determine which version to download
        let version_to_download = match requested_version {
            Some(version) if version != "latest" => version.to_string(),
            _ => {
                // Fetch latest version
                let versions_url = format!("{}/api/packages/{}/versions", base_url, api_package_name);
                let versions_response = client.get(&versions_url).send().await?;

                if versions_response.status() == 404 {
                    anyhow::bail!("Package '{}' not found in Knot Space\n💡 Check the package name spelling\n💡 Verify the package exists at: {}/packages/{}\n💡 Use 'knot search {}' to find similar packages", package_name, base_url, api_package_name, api_package_name);
                } else if !versions_response.status().is_success() {
                    anyhow::bail!("Failed to fetch package '{}' information from Knot Space (HTTP {})\n💡 Check your internet connection\n💡 Try again later if Knot Space is temporarily unavailable", package_name, versions_response.status());
                }

                let versions_data: serde_json::Value = versions_response.json().await?;
                let versions = versions_data["data"]
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("Invalid versions response format"))?;

                if versions.is_empty() {
                    anyhow::bail!("No versions found for package '{}' in Knot Space\n💡 The package exists but has no published versions\n💡 Contact the package maintainer to publish a version\n💡 Check if the package is still in development", package_name);
                }

                versions[0]["version"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Invalid version format"))?
                    .to_string()
            }
        };
        
        // Download the package file
        let download_url = format!("{}/api/packages/{}/{}/download", base_url, api_package_name, version_to_download);
        let download_response = client.get(&download_url).send().await?;
        
        if download_response.status() == 404 {
            anyhow::bail!("Package file not found for '{}' version '{}'\n💡 The version may not exist or was removed\n💡 Check available versions with package information\n💡 Try using 'latest' version instead", package_name, version_to_download);
        } else if !download_response.status().is_success() {
            anyhow::bail!("Failed to download package file for '{}' (HTTP {})\n💡 Check your internet connection\n💡 Try again later if the server is temporarily unavailable\n💡 Contact support if the problem persists", package_name, download_response.status());
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
