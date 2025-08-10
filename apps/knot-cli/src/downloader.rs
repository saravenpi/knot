use anyhow::Result;
use std::fs;
use std::path::Path;

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
        let (team, package) = Self::parse_package_name(package_name)?;

        // For now, we'll simulate downloading from a knot space repository
        // In a real implementation, this would make HTTP requests to a package registry

        println!(
            "📥 Downloading package '{}' from knot space...",
            package_name
        );

        // Create a mock package structure for now
        fs::create_dir_all(destination)?;

        // Create an index file to indicate this is a downloaded package
        let index_file = destination.join("index.js");
        let mock_content = match package_name {
            "@jwt" => "module.exports = { sign: () => {}, verify: () => {} };",
            "@modules-loader" => "module.exports = { load: () => {} };",
            _ if team.is_some() => &format!("// {} package from team {}", package, team.unwrap()),
            _ => &format!("// {} package", package),
        };

        fs::write(index_file, mock_content)?;

        // Create a package.json for the downloaded package
        let package_json = destination.join("package.json");
        let package_json_content = serde_json::json!({
            "name": package_name,
            "version": "1.0.0",
            "main": "index.js"
        });
        fs::write(
            package_json,
            serde_json::to_string_pretty(&package_json_content)?,
        )?;

        println!("✅ Successfully downloaded '{}'", package_name);
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
