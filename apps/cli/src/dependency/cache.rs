use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::dependency::types::{ResolutionRequest, ResolutionResult as TypesResolutionResult};
use crate::dependency::error::{ResolutionError, ResolutionResult};

#[derive(Debug)]
pub struct ResolutionCache {
    memory_cache: HashMap<String, CacheEntry>,
    disk_cache_path: PathBuf,
    cache_ttl: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    result: TypesResolutionResult,
    timestamp: SystemTime,
    request_hash: String,
}

impl ResolutionCache {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            memory_cache: HashMap::new(),
            disk_cache_path: cache_dir.join("resolutions"),
            cache_ttl: Duration::from_secs(3600), // 1 hour default TTL
        }
    }

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.cache_ttl = ttl;
        self
    }

    pub async fn get_cached_resolution(
        &mut self, 
        request: &ResolutionRequest
    ) -> Option<TypesResolutionResult> {
        let request_hash = self.hash_request(request);
        
        // Check memory cache first
        if let Some(entry) = self.memory_cache.get(&request_hash) {
            if !self.is_cache_expired(&entry.timestamp) {
                return Some(entry.result.clone());
            } else {
                // Remove expired entry
                self.memory_cache.remove(&request_hash);
            }
        }
        
        // Check disk cache
        if let Some(entry) = self.load_from_disk_cache(&request_hash).await {
            if !self.is_cache_expired(&entry.timestamp) {
                // Load into memory cache for faster future access
                self.memory_cache.insert(request_hash, entry.clone());
                return Some(entry.result);
            }
        }
        
        None
    }
    
    pub async fn cache_resolution(
        &mut self,
        request: &ResolutionRequest,
        result: &TypesResolutionResult,
    ) -> ResolutionResult<()> {
        let request_hash = self.hash_request(request);
        let entry = CacheEntry {
            result: result.clone(),
            timestamp: SystemTime::now(),
            request_hash: request_hash.clone(),
        };
        
        // Store in memory
        self.memory_cache.insert(request_hash.clone(), entry.clone());
        
        // Store on disk
        self.save_to_disk_cache(&request_hash, &entry).await?;
        
        Ok(())
    }
    
    pub async fn invalidate_package_cache(&mut self, package_name: &str) -> ResolutionResult<()> {
        // Remove from memory cache
        let keys_to_remove: Vec<_> = self.memory_cache.keys()
            .filter(|key| self.cache_contains_package(key, package_name))
            .cloned()
            .collect();
        
        for key in keys_to_remove {
            self.memory_cache.remove(&key);
        }
        
        // Remove from disk cache
        self.cleanup_disk_cache_for_package(package_name).await?;
        
        Ok(())
    }
    
    pub async fn clear_cache(&mut self) -> ResolutionResult<()> {
        // Clear memory cache
        self.memory_cache.clear();
        
        // Clear disk cache
        if self.disk_cache_path.exists() {
            std::fs::remove_dir_all(&self.disk_cache_path)
                .map_err(|e| ResolutionError::cache_error(
                    "clearing disk cache",
                    e.to_string()
                ))?;
        }
        
        Ok(())
    }
    
    pub async fn cleanup_expired(&mut self) -> ResolutionResult<()> {
        // Clean memory cache
        let expired_keys: Vec<_> = self.memory_cache.iter()
            .filter(|(_, entry)| self.is_cache_expired(&entry.timestamp))
            .map(|(key, _)| key.clone())
            .collect();
        
        for key in expired_keys {
            self.memory_cache.remove(&key);
        }
        
        // Clean disk cache
        self.cleanup_expired_disk_cache().await?;
        
        Ok(())
    }

    pub fn cache_stats(&self) -> CacheStats {
        let memory_entries = self.memory_cache.len();
        let memory_size = self.estimate_memory_size();
        
        CacheStats {
            memory_entries,
            memory_size_bytes: memory_size,
            disk_path: self.disk_cache_path.clone(),
            ttl: self.cache_ttl,
        }
    }
    
    fn hash_request(&self, request: &ResolutionRequest) -> String {
        let mut hasher = Sha256::new();
        
        // Hash the dependencies
        for dep in &request.dependencies {
            hasher.update(dep.id.name.as_bytes());
            hasher.update(dep.version_req.to_string().as_bytes());
            hasher.update([dep.optional as u8, dep.dev_only as u8]);
            
            if let Some(conditions) = &dep.conditions {
                if let Some(platform) = &conditions.platform {
                    for p in platform {
                        hasher.update(p.as_bytes());
                    }
                }
                if let Some(arch) = &conditions.arch {
                    for a in arch {
                        hasher.update(a.as_bytes());
                    }
                }
                if let Some(env) = &conditions.env {
                    for e in env {
                        hasher.update(e.as_bytes());
                    }
                }
            }
        }
        
        // Hash the context
        hasher.update(format!("{:?}", request.context.strategy).as_bytes());
        hasher.update([
            request.context.allow_prerelease as u8,
            request.context.include_dev as u8,
            request.context.include_optional as u8,
        ]);
        hasher.update(request.context.max_depth.to_string().as_bytes());
        
        if let Some(platform) = &request.context.platform {
            hasher.update(platform.as_bytes());
        }
        if let Some(arch) = &request.context.arch {
            hasher.update(arch.as_bytes());
        }
        if let Some(env) = &request.context.environment {
            hasher.update(env.as_bytes());
        }
        
        // Hash overrides and excludes
        for (package_id, version) in &request.overrides {
            hasher.update(package_id.name.as_bytes());
            hasher.update(version.to_string().as_bytes());
        }
        
        for package_id in &request.excludes {
            hasher.update(package_id.name.as_bytes());
        }
        
        format!("{:x}", hasher.finalize())
    }
    
    fn is_cache_expired(&self, timestamp: &SystemTime) -> bool {
        timestamp.elapsed().unwrap_or(Duration::MAX) > self.cache_ttl
    }
    
    async fn load_from_disk_cache(&self, request_hash: &str) -> Option<CacheEntry> {
        let cache_file = self.disk_cache_path.join(format!("{}.json", request_hash));
        
        if !cache_file.exists() {
            return None;
        }
        
        let content = tokio::fs::read_to_string(&cache_file).await.ok()?;
        serde_json::from_str::<CacheEntry>(&content).ok()
    }
    
    async fn save_to_disk_cache(&self, request_hash: &str, entry: &CacheEntry) -> ResolutionResult<()> {
        // Ensure cache directory exists
        tokio::fs::create_dir_all(&self.disk_cache_path).await
            .map_err(|e| ResolutionError::cache_error(
                "creating cache directory",
                e.to_string()
            ))?;
        
        let cache_file = self.disk_cache_path.join(format!("{}.json", request_hash));
        let content = serde_json::to_string_pretty(entry)
            .map_err(|e| ResolutionError::cache_error(
                "serializing cache entry",
                e.to_string()
            ))?;
        
        tokio::fs::write(&cache_file, content).await
            .map_err(|e| ResolutionError::cache_error(
                "writing cache file",
                e.to_string()
            ))?;
        
        Ok(())
    }
    
    fn cache_contains_package(&self, cache_key: &str, package_name: &str) -> bool {
        // This is a simplified check - in practice, you'd want to decode
        // the hash or store metadata to make this more efficient
        if let Some(entry) = self.memory_cache.get(cache_key) {
            entry.result.resolved_packages.keys()
                .any(|id| id.name.contains(package_name))
        } else {
            false
        }
    }
    
    async fn cleanup_disk_cache_for_package(&self, package_name: &str) -> ResolutionResult<()> {
        if !self.disk_cache_path.exists() {
            return Ok(());
        }
        
        let mut entries = tokio::fs::read_dir(&self.disk_cache_path).await
            .map_err(|e| ResolutionError::cache_error(
                "reading cache directory",
                e.to_string()
            ))?;
        
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| ResolutionError::cache_error(
                "reading cache entry",
                e.to_string()
            ))? {
            
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                // Check if this cache entry contains the package
                if let Ok(content) = tokio::fs::read_to_string(&path).await {
                    if let Ok(cache_entry) = serde_json::from_str::<CacheEntry>(&content) {
                        if cache_entry.result.resolved_packages.keys()
                            .any(|id| id.name.contains(package_name)) {
                            let _ = tokio::fs::remove_file(&path).await;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    async fn cleanup_expired_disk_cache(&self) -> ResolutionResult<()> {
        if !self.disk_cache_path.exists() {
            return Ok(());
        }
        
        let mut entries = tokio::fs::read_dir(&self.disk_cache_path).await
            .map_err(|e| ResolutionError::cache_error(
                "reading cache directory",
                e.to_string()
            ))?;
        
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| ResolutionError::cache_error(
                "reading cache entry",
                e.to_string()
            ))? {
            
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                // Check if this cache entry is expired
                if let Ok(content) = tokio::fs::read_to_string(&path).await {
                    if let Ok(cache_entry) = serde_json::from_str::<CacheEntry>(&content) {
                        if self.is_cache_expired(&cache_entry.timestamp) {
                            let _ = tokio::fs::remove_file(&path).await;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn estimate_memory_size(&self) -> usize {
        // This is a rough estimate - in practice you'd want more accurate measurement
        self.memory_cache.len() * 1024 // Assume ~1KB per entry on average
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub memory_entries: usize,
    pub memory_size_bytes: usize,
    pub disk_path: PathBuf,
    pub ttl: Duration,
}

impl CacheStats {
    pub fn format_size(&self) -> String {
        if self.memory_size_bytes < 1024 {
            format!("{} B", self.memory_size_bytes)
        } else if self.memory_size_bytes < 1024 * 1024 {
            format!("{:.1} KB", self.memory_size_bytes as f64 / 1024.0)
        } else {
            format!("{:.1} MB", self.memory_size_bytes as f64 / (1024.0 * 1024.0))
        }
    }
    
    pub fn format_ttl(&self) -> String {
        let secs = self.ttl.as_secs();
        if secs < 60 {
            format!("{}s", secs)
        } else if secs < 3600 {
            format!("{}m", secs / 60)
        } else {
            format!("{}h", secs / 3600)
        }
    }
}