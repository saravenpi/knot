use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::sync::RwLock;

use crate::dependency::types::{ResolutionRequest, ResolutionResult as TypesResolutionResult, PackageId};
use crate::dependency::error::{ResolutionError, ResolutionResult};

#[derive(Debug)]
pub struct ResolutionCache {
    memory_cache: RwLock<HashMap<String, CacheEntry>>,
    package_cache: RwLock<HashMap<PackageId, PackageCacheEntry>>,
    disk_cache_path: PathBuf,
    cache_ttl: Duration,
    max_memory_entries: usize,
    max_memory_size: usize, // in bytes
    cache_statistics: RwLock<CacheStatistics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    result: TypesResolutionResult,
    timestamp: SystemTime,
    request_hash: String,
    access_count: u64,
    last_access: SystemTime,
    size_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PackageCacheEntry {
    versions: Vec<crate::dependency::types::PackageVersion>,
    timestamp: SystemTime,
    last_access: SystemTime,
    source_fingerprint: String, // Hash of source directory for local packages
}

#[derive(Debug, Clone)]
struct CacheStatistics {
    hits: u64,
    misses: u64,
    evictions: u64,
    total_requests: u64,
    memory_usage: usize,
}

impl ResolutionCache {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            memory_cache: RwLock::new(HashMap::new()),
            package_cache: RwLock::new(HashMap::new()),
            disk_cache_path: cache_dir.join("resolutions"),
            cache_ttl: Duration::from_secs(3600), // 1 hour default TTL
            max_memory_entries: 1000,
            max_memory_size: 100 * 1024 * 1024, // 100MB
            cache_statistics: RwLock::new(CacheStatistics {
                hits: 0,
                misses: 0,
                evictions: 0,
                total_requests: 0,
                memory_usage: 0,
            }),
        }
    }

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.cache_ttl = ttl;
        self
    }

    pub fn with_memory_limits(mut self, max_entries: usize, max_size: usize) -> Self {
        self.max_memory_entries = max_entries;
        self.max_memory_size = max_size;
        self
    }

    pub async fn get_cached_resolution(
        &self, 
        request: &ResolutionRequest
    ) -> Option<TypesResolutionResult> {
        let request_hash = self.hash_request(request);
        
        // Update statistics
        {
            let mut stats = self.cache_statistics.write().await;
            stats.total_requests += 1;
        }
        
        // Check memory cache first
        {
            let mut memory_cache = self.memory_cache.write().await;
            if let Some(entry) = memory_cache.get_mut(&request_hash) {
                if !self.is_cache_expired(&entry.timestamp) {
                    // Update access statistics
                    entry.access_count += 1;
                    entry.last_access = SystemTime::now();
                    
                    // Update cache statistics
                    let mut stats = self.cache_statistics.write().await;
                    stats.hits += 1;
                    
                    return Some(entry.result.clone());
                } else {
                    // Remove expired entry
                    memory_cache.remove(&request_hash);
                }
            }
        }
        
        // Check disk cache
        if let Some(mut entry) = self.load_from_disk_cache(&request_hash).await {
            if !self.is_cache_expired(&entry.timestamp) {
                // Update access info
                entry.access_count += 1;
                entry.last_access = SystemTime::now();
                
                // Load into memory cache for faster future access
                let result = entry.result.clone();
                self.insert_into_memory_cache(request_hash, entry).await;
                
                // Update cache statistics
                let mut stats = self.cache_statistics.write().await;
                stats.hits += 1;
                
                return Some(result);
            }
        }
        
        // Cache miss
        {
            let mut stats = self.cache_statistics.write().await;
            stats.misses += 1;
        }
        
        None
    }
    
    pub async fn cache_resolution(
        &self,
        request: &ResolutionRequest,
        result: &TypesResolutionResult,
    ) -> ResolutionResult<()> {
        let request_hash = self.hash_request(request);
        let size_bytes = self.estimate_entry_size(result);
        
        let entry = CacheEntry {
            result: result.clone(),
            timestamp: SystemTime::now(),
            request_hash: request_hash.clone(),
            access_count: 1,
            last_access: SystemTime::now(),
            size_bytes,
        };
        
        // Store in memory with eviction policy
        self.insert_into_memory_cache(request_hash.clone(), entry.clone()).await;
        
        // Store on disk
        self.save_to_disk_cache(&request_hash, &entry).await?;
        
        Ok(())
    }
    
    pub async fn invalidate_package_cache(&self, package_name: &str) -> ResolutionResult<()> {
        // Remove from memory cache
        {
            let mut memory_cache = self.memory_cache.write().await;
            let keys_to_remove: Vec<_> = memory_cache.keys()
                .filter(|key| self.cache_contains_package_by_hash(key, package_name))
                .cloned()
                .collect();
            
            for key in keys_to_remove {
                memory_cache.remove(&key);
            }
        }
        
        // Remove from package cache
        {
            let mut package_cache = self.package_cache.write().await;
            let keys_to_remove: Vec<_> = package_cache.keys()
                .filter(|id| id.name.contains(package_name))
                .cloned()
                .collect();
            
            for key in keys_to_remove {
                package_cache.remove(&key);
            }
        }
        
        // Remove from disk cache
        self.cleanup_disk_cache_for_package(package_name).await?;
        
        Ok(())
    }
    
    pub async fn clear_cache(&self) -> ResolutionResult<()> {
        // Clear memory caches
        {
            let mut memory_cache = self.memory_cache.write().await;
            memory_cache.clear();
        }
        {
            let mut package_cache = self.package_cache.write().await;
            package_cache.clear();
        }
        
        // Reset statistics
        {
            let mut stats = self.cache_statistics.write().await;
            *stats = CacheStatistics {
                hits: 0,
                misses: 0,
                evictions: 0,
                total_requests: 0,
                memory_usage: 0,
            };
        }
        
        // Clear disk cache
        if self.disk_cache_path.exists() {
            tokio::fs::remove_dir_all(&self.disk_cache_path).await
                .map_err(|e| ResolutionError::cache_error(
                    "clearing disk cache",
                    e.to_string()
                ))?;
        }
        
        Ok(())
    }
    
    pub async fn cleanup_expired(&self) -> ResolutionResult<()> {
        // Clean memory cache
        {
            let mut memory_cache = self.memory_cache.write().await;
            let expired_keys: Vec<_> = memory_cache.iter()
                .filter(|(_, entry)| self.is_cache_expired(&entry.timestamp))
                .map(|(key, _)| key.clone())
                .collect();
            
            for key in expired_keys {
                memory_cache.remove(&key);
            }
        }
        
        // Clean package cache
        {
            let mut package_cache = self.package_cache.write().await;
            let expired_keys: Vec<_> = package_cache.iter()
                .filter(|(_, entry)| self.is_cache_expired(&entry.timestamp))
                .map(|(key, _)| key.clone())
                .collect();
            
            for key in expired_keys {
                package_cache.remove(&key);
            }
        }
        
        // Clean disk cache
        self.cleanup_expired_disk_cache().await?;
        
        Ok(())
    }

    pub async fn cache_stats(&self) -> CacheStats {
        let memory_cache = self.memory_cache.read().await;
        let package_cache = self.package_cache.read().await;
        let stats = self.cache_statistics.read().await;
        
        let memory_entries = memory_cache.len();
        let package_entries = package_cache.len();
        let memory_size = stats.memory_usage;
        
        let hit_rate = if stats.total_requests > 0 {
            (stats.hits as f64) / (stats.total_requests as f64) * 100.0
        } else {
            0.0
        };
        
        CacheStats {
            memory_entries,
            package_entries,
            memory_size_bytes: memory_size,
            disk_path: self.disk_cache_path.clone(),
            ttl: self.cache_ttl,
            hit_rate,
            total_hits: stats.hits,
            total_misses: stats.misses,
            total_evictions: stats.evictions,
            total_requests: stats.total_requests,
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
    
    fn cache_contains_package_by_hash(&self, _cache_key: &str, _package_name: &str) -> bool {
        // This is a simplified check - for better performance, we'd need to store
        // metadata about which packages are in each cache entry
        // For now, we'll use the package cache invalidation instead
        false
    }

    async fn insert_into_memory_cache(&self, key: String, entry: CacheEntry) {
        let mut memory_cache = self.memory_cache.write().await;
        
        // Check if we need to evict entries
        if memory_cache.len() >= self.max_memory_entries {
            self.evict_lru_entries(&mut memory_cache, 1).await;
        }
        
        // Check memory usage
        let current_usage = self.calculate_memory_usage(&memory_cache);
        if current_usage + entry.size_bytes > self.max_memory_size {
            // Evict entries until we have space
            let target_size = self.max_memory_size - entry.size_bytes;
            self.evict_to_size(&mut memory_cache, target_size).await;
        }
        
        memory_cache.insert(key, entry);
        
        // Update memory usage statistics
        let mut stats = self.cache_statistics.write().await;
        stats.memory_usage = self.calculate_memory_usage(&memory_cache);
    }

    async fn evict_lru_entries(&self, memory_cache: &mut HashMap<String, CacheEntry>, count: usize) {
        let mut entries: Vec<_> = memory_cache.iter().collect();
        entries.sort_by_key(|(_, entry)| entry.last_access);
        
        for i in 0..count.min(entries.len()) {
            let key = entries[i].0.clone();
            memory_cache.remove(&key);
        }
        
        // Update eviction statistics
        let mut stats = self.cache_statistics.write().await;
        stats.evictions += count as u64;
    }

    async fn evict_to_size(&self, memory_cache: &mut HashMap<String, CacheEntry>, target_size: usize) {
        let mut entries: Vec<_> = memory_cache.iter().collect();
        entries.sort_by_key(|(_, entry)| entry.last_access);
        
        let mut current_size = self.calculate_memory_usage(memory_cache);
        let mut evicted = 0;
        
        for (key, entry) in entries {
            if current_size <= target_size {
                break;
            }
            
            current_size -= entry.size_bytes;
            memory_cache.remove(key);
            evicted += 1;
        }
        
        // Update eviction statistics
        let mut stats = self.cache_statistics.write().await;
        stats.evictions += evicted;
    }

    fn calculate_memory_usage(&self, memory_cache: &HashMap<String, CacheEntry>) -> usize {
        memory_cache.values().map(|entry| entry.size_bytes).sum()
    }

    fn estimate_entry_size(&self, result: &TypesResolutionResult) -> usize {
        // Rough estimate of memory usage for cache entry
        let base_size = std::mem::size_of::<CacheEntry>();
        let packages_size = result.resolved_packages.len() * 512; // Rough estimate per package
        let order_size = result.dependency_order.len() * 64; // Rough estimate per package ID
        
        base_size + packages_size + order_size
    }

    // Package-specific caching methods
    pub async fn cache_package_versions(&self, package_id: &PackageId, versions: Vec<crate::dependency::types::PackageVersion>, fingerprint: String) {
        let entry = PackageCacheEntry {
            versions,
            timestamp: SystemTime::now(),
            last_access: SystemTime::now(),
            source_fingerprint: fingerprint,
        };
        
        let mut package_cache = self.package_cache.write().await;
        package_cache.insert(package_id.clone(), entry);
    }

    pub async fn get_cached_package_versions(&self, package_id: &PackageId, current_fingerprint: &str) -> Option<Vec<crate::dependency::types::PackageVersion>> {
        let mut package_cache = self.package_cache.write().await;
        
        if let Some(entry) = package_cache.get_mut(package_id) {
            if !self.is_cache_expired(&entry.timestamp) && entry.source_fingerprint == current_fingerprint {
                entry.last_access = SystemTime::now();
                return Some(entry.versions.clone());
            } else {
                // Remove stale entry
                package_cache.remove(package_id);
            }
        }
        
        None
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
    pub package_entries: usize,
    pub memory_size_bytes: usize,
    pub disk_path: PathBuf,
    pub ttl: Duration,
    pub hit_rate: f64,
    pub total_hits: u64,
    pub total_misses: u64,
    pub total_evictions: u64,
    pub total_requests: u64,
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
    
    pub fn efficiency_score(&self) -> f64 {
        // Calculate cache efficiency based on hit rate and memory usage efficiency
        let hit_rate_score = self.hit_rate / 100.0; // Normalize to 0-1
        let memory_efficiency = if self.memory_entries > 0 {
            1.0 - (self.total_evictions as f64 / self.memory_entries as f64).min(1.0)
        } else {
            1.0
        };
        
        (hit_rate_score * 0.7) + (memory_efficiency * 0.3)
    }
    
    pub fn print_detailed_report(&self) {
        use console::style;
        
        println!("\n📊 Cache Performance Report:");
        println!("  Memory Entries: {}", style(self.memory_entries).cyan());
        println!("  Package Entries: {}", style(self.package_entries).cyan());
        println!("  Memory Usage: {}", style(self.format_size()).yellow());
        println!("  TTL: {}", style(self.format_ttl()).yellow());
        println!("  Hit Rate: {:.1}%", style(format!("{:.1}", self.hit_rate)).green());
        println!("  Total Requests: {}", self.total_requests);
        println!("  Cache Hits: {}", style(self.total_hits).green());
        println!("  Cache Misses: {}", style(self.total_misses).red());
        println!("  Evictions: {}", style(self.total_evictions).yellow());
        println!("  Efficiency Score: {:.2}/1.0", style(format!("{:.2}", self.efficiency_score())).cyan());
        
        // Performance indicators
        if self.hit_rate >= 80.0 {
            println!("  ✅ Excellent cache performance");
        } else if self.hit_rate >= 60.0 {
            println!("  ⚠️  Good cache performance, consider increasing TTL");
        } else {
            println!("  ❌ Poor cache performance, review caching strategy");
        }
    }
}