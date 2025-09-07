use console::style;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct InstallationStats {
    pub total_packages: usize,
    pub local_installs: usize,
    pub remote_installs: usize,
    pub failed_installs: Vec<(String, String)>, // (package_name, error)
    pub total_duration: std::time::Duration,
    pub download_duration: std::time::Duration,
    pub link_duration: std::time::Duration,
    pub cache_hits: usize,
    pub verified_checksums: usize,
}

impl InstallationStats {
    pub fn new() -> Self {
        Self {
            total_packages: 0,
            local_installs: 0,
            remote_installs: 0,
            failed_installs: Vec::new(),
            total_duration: std::time::Duration::ZERO,
            download_duration: std::time::Duration::ZERO,
            link_duration: std::time::Duration::ZERO,
            cache_hits: 0,
            verified_checksums: 0,
        }
    }
    
    pub fn record_local_install(&mut self, duration: std::time::Duration) {
        self.total_packages += 1;
        self.local_installs += 1;
        self.total_duration += duration;
        self.link_duration += duration;
    }
    
    pub fn record_remote_install(&mut self, duration: std::time::Duration, checksum: Option<String>) {
        self.total_packages += 1;
        self.remote_installs += 1;
        self.total_duration += duration;
        self.download_duration += duration;
        
        if checksum.is_some() {
            self.verified_checksums += 1;
        }
    }
    
    pub fn record_failed_install(&mut self, package_name: String, error: String) {
        self.failed_installs.push((package_name, error));
    }
    
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.total_packages == 0 {
            return 100.0;
        }
        
        let successful = self.total_packages - self.failed_installs.len();
        (successful as f64 / self.total_packages as f64) * 100.0
    }
    
    pub fn average_install_time(&self) -> std::time::Duration {
        if self.total_packages == 0 {
            return std::time::Duration::ZERO;
        }
        
        self.total_duration / self.total_packages as u32
    }
    
    pub fn print_summary(&self) {
        println!("\n📊 Installation Statistics:");
        println!("  Total packages: {}", style(self.total_packages).cyan());
        println!("  Local packages: {} ({}%)", 
                 style(self.local_installs).green(), 
                 if self.total_packages > 0 { (self.local_installs * 100) / self.total_packages } else { 0 });
        println!("  Remote packages: {} ({}%)", 
                 style(self.remote_installs).blue(), 
                 if self.total_packages > 0 { (self.remote_installs * 100) / self.total_packages } else { 0 });
        println!("  Cache hits: {}", style(self.cache_hits).yellow());
        println!("  Verified checksums: {}", style(self.verified_checksums).green());
        println!("  Success rate: {:.1}%", style(format!("{:.1}", self.success_rate())).green());
        println!("  Total time: {:.2}s", style(format!("{:.2}", self.total_duration.as_secs_f64())).cyan());
        println!("  Average per package: {:.2}s", style(format!("{:.2}", self.average_install_time().as_secs_f64())).cyan());
        
        if !self.failed_installs.is_empty() {
            println!("  ❌ Failed installations:");
            for (package_name, error) in &self.failed_installs {
                println!("    {} - {}", style(package_name).red(), style(error).dim());
            }
        }
        
        // Performance insights
        if self.total_packages > 0 {
            println!("\n💡 Performance Insights:");
            
            if self.cache_hits > 0 {
                println!("  ✅ Cache is working efficiently");
            } else if self.remote_installs > 5 {
                println!("  💡 Consider pre-warming cache for better performance");
            }
            
            if self.download_duration > self.link_duration * 2 {
                println!("  💡 Remote downloads are the bottleneck - consider using a local mirror");
            }
            
            if self.average_install_time().as_secs_f64() > 5.0 {
                println!("  ⚠️  Slow installation times detected - check network connection");
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstallationLogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug)]
pub struct InstallationReport {
    pub app_name: String,
    pub success: bool,
    pub packages_installed: usize,
    pub total_duration: std::time::Duration,
    pub cache_hits: usize,
    pub failed_packages: Vec<(String, String)>,
    pub error: Option<String>,
    pub timestamp: std::time::SystemTime,
}

impl InstallationReport {
    pub fn new(app_name: String) -> Self {
        Self {
            app_name,
            success: false,
            packages_installed: 0,
            total_duration: std::time::Duration::ZERO,
            cache_hits: 0,
            failed_packages: Vec::new(),
            error: None,
            timestamp: std::time::SystemTime::now(),
        }
    }
    
    pub fn print_report(&self) {
        println!("\n📊 Installation Report for app '{}':", style(&self.app_name).green());
        println!("  Status: {}", if self.success { 
            style("Success").green() 
        } else { 
            style("Failed").red() 
        });
        println!("  Packages installed: {}", style(self.packages_installed).cyan());
        println!("  Duration: {:.2}s", style(format!("{:.2}", self.total_duration.as_secs_f64())).cyan());
        println!("  Cache hits: {}", style(self.cache_hits).yellow());
        
        if !self.failed_packages.is_empty() {
            println!("  Failed packages:");
            for (name, error) in &self.failed_packages {
                println!("    {} - {}", style(name).red(), style(error).dim());
            }
        }
        
        if let Some(error) = &self.error {
            println!("  Error: {}", style(error).red());
        }
        
        println!("  Timestamp: {}", 
                 style(format!("{:?}", self.timestamp)).dim());
    }
}

#[derive(Debug)]
pub struct InstallationValidationReport {
    pub app_name: String,
    pub is_valid: bool,
    pub expected_packages: usize,
    pub valid_packages: usize,
    pub missing_packages: Vec<String>,
    pub corrupted_packages: Vec<String>,
    pub issues: Vec<String>,
}

impl InstallationValidationReport {
    pub fn new(app_name: String) -> Self {
        Self {
            app_name,
            is_valid: true,
            expected_packages: 0,
            valid_packages: 0,
            missing_packages: Vec::new(),
            corrupted_packages: Vec::new(),
            issues: Vec::new(),
        }
    }
    
    pub fn print_report(&self) {
        println!("\n🔍 Installation Validation Report for app '{}':", style(&self.app_name).green());
        println!("  Status: {}", if self.is_valid { 
            style("Valid").green() 
        } else { 
            style("Invalid").red() 
        });
        println!("  Expected packages: {}", style(self.expected_packages).cyan());
        println!("  Valid packages: {}", style(self.valid_packages).green());
        
        if !self.missing_packages.is_empty() {
            println!("  Missing packages: {}", style(self.missing_packages.len()).red());
            for package in &self.missing_packages {
                println!("    - {}", style(package).red());
            }
        }
        
        if !self.corrupted_packages.is_empty() {
            println!("  Corrupted packages: {}", style(self.corrupted_packages.len()).yellow());
            for package in &self.corrupted_packages {
                println!("    - {}", style(package).yellow());
            }
        }
        
        if !self.issues.is_empty() {
            println!("  Issues:");
            for issue in &self.issues {
                println!("    - {}", issue);
            }
        }
    }
}