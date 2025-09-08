use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;
use semver::{Version, VersionReq};

use crate::dependency::types::{
    PackageId, PackageVersion, DependencySpec, ResolutionContext, ResolutionStrategy,
    DependencyGraph, ResolutionRequest, ResolutionResult as TypesResolutionResult,
    DependencyConflict,
};
use crate::dependency::error::{ResolutionError, ResolutionResult};
use crate::dependency::registry::{PackageRegistry, LocalPackageRegistry, RemotePackageRegistry};
use crate::dependency::cache::ResolutionCache;

pub struct DependencyResolver {
    context: ResolutionContext,
    local_registry: LocalPackageRegistry,
    remote_registry: RemotePackageRegistry,
    cache: ResolutionCache,
}

impl DependencyResolver {
    pub fn new(
        context: ResolutionContext,
        local_registry: LocalPackageRegistry,
        remote_registry: RemotePackageRegistry,
        cache_dir: PathBuf,
    ) -> Self {
        Self {
            context,
            local_registry,
            remote_registry,
            cache: ResolutionCache::new(cache_dir),
        }
    }

    pub async fn resolve_dependencies(
        &mut self,
        root_dependencies: Vec<DependencySpec>,
    ) -> ResolutionResult<TypesResolutionResult> {
        let request = ResolutionRequest {
            dependencies: root_dependencies.clone(),
            context: self.context.clone(),
            overrides: HashMap::new(),
            excludes: Vec::new(),
        };

        // Check cache first
        if let Some(cached_result) = self.cache.get_cached_resolution(&request).await {
            return Ok(cached_result);
        }

        let mut graph = DependencyGraph::new();
        
        // Phase 1: Discover all packages and versions
        self.discover_phase(&mut graph, &root_dependencies).await?;
        
        // Phase 2: Build constraint graph
        self.constraint_phase(&mut graph, &root_dependencies)?;
        
        // Phase 3: Resolve conflicts and select versions
        let resolution = self.resolution_phase(&mut graph)?;
        
        // Phase 4: Generate dependency order
        let dependency_order = self.topological_sort(&resolution)?;
        
        // Phase 5: Validate resolution
        let conflicts = self.detect_remaining_conflicts(&resolution);
        let warnings = self.generate_warnings(&resolution);
        
        let result = TypesResolutionResult {
            resolved_packages: resolution,
            dependency_order,
            conflicts,
            warnings,
            lock_file_hash: None,
        };

        // Cache the result
        self.cache.cache_resolution(&request, &result).await?;

        Ok(result)
    }

    async fn discover_phase(
        &mut self,
        graph: &mut DependencyGraph,
        root_deps: &[DependencySpec],
    ) -> ResolutionResult<()> {
        let mut queue = VecDeque::from(root_deps.to_vec());
        let mut discovered = HashSet::new();

        // Discover local packages
        self.local_registry.discover_packages().await?;

        while let Some(dep_spec) = queue.pop_front() {
            if !dep_spec.is_applicable(&self.context) {
                continue;
            }

            if discovered.contains(&dep_spec.id) {
                continue;
            }
            discovered.insert(dep_spec.id.clone());

            // Discover available versions for this package
            let versions = self.discover_package_versions(&dep_spec.id).await?;
            
            if versions.is_empty() {
                return Err(ResolutionError::package_not_found(
                    dep_spec.id.clone(),
                    vec!["Local packages".to_string(), "Remote registry".to_string()],
                    self.find_similar_packages(&dep_spec.id.name).await,
                ));
            }

            graph.packages.insert(dep_spec.id.clone(), versions);

            // Queue transitive dependencies for discovery
            if let Some(package_versions) = graph.packages.get(&dep_spec.id) {
                for version in package_versions {
                    if dep_spec.version_req.matches(&version.version) {
                        let transitive_deps = version.get_applicable_dependencies(&self.context);
                        for transitive_dep in transitive_deps {
                            if !discovered.contains(&transitive_dep.id) {
                                queue.push_back(transitive_dep.clone());
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn constraint_phase(
        &mut self,
        graph: &mut DependencyGraph,
        root_deps: &[DependencySpec],
    ) -> ResolutionResult<()> {
        let mut work_queue = VecDeque::new();

        // Add root constraints
        for dep_spec in root_deps {
            if dep_spec.is_applicable(&self.context) {
                self.add_constraint(graph, &dep_spec.id, &dep_spec.version_req);
                work_queue.push_back(dep_spec.id.clone());
            }
        }

        // Process transitive constraints
        while let Some(package_id) = work_queue.pop_front() {
            let versions_clone = graph.packages.get(&package_id).cloned();
            if let Some(versions) = versions_clone {
                for version in versions {
                    if self.version_satisfies_constraints(graph, &package_id, &version.version) {
                        let deps = version.get_applicable_dependencies(&self.context);
                        for dep in deps {
                            let had_constraint = graph.constraints.contains_key(&dep.id);
                            self.add_constraint(graph, &dep.id, &dep.version_req);
                            
                            // Only queue if this is a new constraint
                            if !had_constraint {
                                work_queue.push_back(dep.id.clone());
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn resolution_phase(&mut self, graph: &mut DependencyGraph) -> ResolutionResult<HashMap<PackageId, PackageVersion>> {
        let mut resolution = HashMap::new();
        
        // Sort packages by dependency order to resolve in correct sequence
        let sorted_packages = self.dependency_order_for_resolution(&graph.packages)?;
        
        for package_id in sorted_packages {
            let selected_version = self.select_version(graph, &package_id)?;
            resolution.insert(package_id.clone(), selected_version);
        }

        // Check for circular dependencies
        self.detect_cycles(&resolution)?;

        Ok(resolution)
    }

    fn select_version(
        &self,
        graph: &DependencyGraph,
        package_id: &PackageId,
    ) -> ResolutionResult<PackageVersion> {
        let versions = graph.packages.get(package_id)
            .ok_or_else(|| ResolutionError::package_not_found(
                package_id.clone(),
                vec!["Dependency graph".to_string()],
                vec![]
            ))?;

        let empty_constraints = Vec::new();
        let constraints = graph.constraints.get(package_id).unwrap_or(&empty_constraints);

        let compatible_versions: Vec<_> = versions
            .iter()
            .filter(|v| {
                constraints.iter().all(|req| req.matches(&v.version)) &&
(v.version.pre.is_empty() || self.context.allow_prerelease)
            })
            .collect();

        if compatible_versions.is_empty() {
            let conflicting_requirements: Vec<_> = constraints.iter()
                .map(|req| (req.clone(), PackageId::local("unknown")))
                .collect();
            
            return Err(ResolutionError::version_conflict(
                package_id.clone(),
                conflicting_requirements,
                Some("Try relaxing version constraints or updating to compatible versions".to_string())
            ));
        }

        // Apply resolution strategy
        let selected = match self.context.strategy {
            ResolutionStrategy::Latest => {
                compatible_versions.iter().max_by(|a, b| a.version.cmp(&b.version)).copied()
            }
            ResolutionStrategy::Strict => {
                // For strict mode, require exact matches
                if constraints.len() == 1 && constraints[0].to_string().chars().all(|c| c.is_ascii_digit() || c == '.') {
                    compatible_versions.first().copied()
                } else {
                    return Err(ResolutionError::configuration_error(
                        "Strict mode requires exact version specifications",
                        Some(format!("package: {}", package_id.name))
                    ));
                }
            }
            ResolutionStrategy::Compatible => {
                // Prefer latest patch within compatible major.minor
                self.select_compatible_version(&compatible_versions)
            }
            ResolutionStrategy::Conservative => {
                // Prefer oldest compatible version
                compatible_versions.iter().min_by(|a, b| a.version.cmp(&b.version)).copied()
            }
        }.ok_or_else(|| ResolutionError::configuration_error(
            "No version could be selected with current strategy",
            Some(format!("package: {}, strategy: {:?}", package_id.name, self.context.strategy))
        ))?;

        Ok(selected.clone())
    }

    fn select_compatible_version<'a>(&self, versions: &[&'a PackageVersion]) -> Option<&'a PackageVersion> {
        // Group by major.minor version
        let mut version_groups: HashMap<(u64, u64), Vec<&'a PackageVersion>> = HashMap::new();
        
        for version in versions {
            let key = (version.version.major, version.version.minor);
            version_groups.entry(key).or_default().push(version);
        }

        // Find the group with the highest major.minor
        let latest_group = version_groups.iter()
            .max_by_key(|(key, _)| *key)?;

        // Within that group, find the highest patch version
        latest_group.1.iter()
            .max_by(|a, b| a.version.patch.cmp(&b.version.patch)).copied()
    }

    fn topological_sort(&self, resolution: &HashMap<PackageId, PackageVersion>) -> ResolutionResult<Vec<PackageId>> {
        let mut in_degree = HashMap::new();
        let mut graph_edges = HashMap::new();
        let mut all_nodes = HashSet::new();

        // Build the graph
        for (package_id, package_version) in resolution {
            all_nodes.insert(package_id.clone());
            in_degree.entry(package_id.clone()).or_insert(0);
            
            let deps = package_version.get_applicable_dependencies(&self.context);
            let dep_ids: Vec<_> = deps.iter().map(|d| d.id.clone()).collect();
            
            graph_edges.insert(package_id.clone(), dep_ids.clone());
            
            for dep_id in dep_ids {
                all_nodes.insert(dep_id.clone());
                *in_degree.entry(dep_id).or_insert(0) += 1;
            }
        }

        // Kahn's algorithm
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        // Find all nodes with no incoming edges
        for (node, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(node.clone());
            }
        }

        while let Some(node) = queue.pop_front() {
            result.push(node.clone());

            if let Some(neighbors) = graph_edges.get(&node) {
                for neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        if result.len() != all_nodes.len() {
            let remaining: Vec<_> = all_nodes.iter()
                .filter(|id| !result.contains(id))
                .collect();
            
            return Err(ResolutionError::circular_dependency(
                remaining.into_iter().cloned().collect()
            ));
        }

        Ok(result)
    }

    fn detect_cycles(&self, resolution: &HashMap<PackageId, PackageVersion>) -> ResolutionResult<()> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for package_id in resolution.keys() {
            if !visited.contains(package_id) {
                if let Some(cycle) = self.dfs_cycle_detect(
                    package_id,
                    resolution,
                    &mut visited,
                    &mut rec_stack,
                    &mut path,
                ) {
                    return Err(ResolutionError::circular_dependency(cycle));
                }
            }
        }

        Ok(())
    }

    fn dfs_cycle_detect(
        &self,
        node: &PackageId,
        resolution: &HashMap<PackageId, PackageVersion>,
        visited: &mut HashSet<PackageId>,
        rec_stack: &mut HashSet<PackageId>,
        path: &mut Vec<PackageId>,
    ) -> Option<Vec<PackageId>> {
        visited.insert(node.clone());
        rec_stack.insert(node.clone());
        path.push(node.clone());

        if let Some(package_version) = resolution.get(node) {
            let deps = package_version.get_applicable_dependencies(&self.context);
            
            for dep in deps {
                if !visited.contains(&dep.id) {
                    if let Some(cycle) = self.dfs_cycle_detect(&dep.id, resolution, visited, rec_stack, path) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(&dep.id) {
                    // Found a cycle - extract the cycle from the path
                    if let Some(cycle_start) = path.iter().position(|id| id == &dep.id) {
                        let mut cycle = path[cycle_start..].to_vec();
                        cycle.push(dep.id.clone()); // Close the cycle
                        return Some(cycle);
                    }
                }
            }
        }

        rec_stack.remove(node);
        path.pop();
        None
    }

    async fn discover_package_versions(&self, package_id: &PackageId) -> ResolutionResult<Vec<PackageVersion>> {
        match &package_id.source {
            crate::dependency::types::PackageSource::Local => {
                self.local_registry.list_versions(package_id).await
            }
            crate::dependency::types::PackageSource::Remote { .. } => {
                self.remote_registry.list_versions(package_id).await
            }
        }
    }

    async fn find_similar_packages(&self, query: &str) -> Vec<String> {
        let mut similar = Vec::new();

        // Search local packages
        if let Ok(local_matches) = self.local_registry.search_packages(query).await {
            similar.extend(local_matches);
        }

        // Search remote packages
        if let Ok(remote_matches) = self.remote_registry.search_packages(query).await {
            similar.extend(remote_matches);
        }

        // Use fuzzy matching to find similar names
        similar.sort_by(|a, b| {
            let a_distance = edit_distance(query, a);
            let b_distance = edit_distance(query, b);
            a_distance.cmp(&b_distance)
        });

        similar.truncate(5); // Limit to top 5 suggestions
        similar
    }

    fn add_constraint(&self, graph: &mut DependencyGraph, package_id: &PackageId, version_req: &VersionReq) {
        graph.constraints.entry(package_id.clone()).or_default().push(version_req.clone());
    }

    fn version_satisfies_constraints(&self, graph: &DependencyGraph, package_id: &PackageId, version: &Version) -> bool {
        if let Some(constraints) = graph.constraints.get(package_id) {
            constraints.iter().all(|req| req.matches(version))
        } else {
            true
        }
    }

    fn dependency_order_for_resolution(&self, packages: &HashMap<PackageId, Vec<PackageVersion>>) -> ResolutionResult<Vec<PackageId>> {
        // For resolution, we need to process packages in an order where dependencies are resolved first
        let mut result = Vec::new();
        let mut visited = HashSet::new();

        for package_id in packages.keys() {
            if !visited.contains(package_id) {
                self.visit_for_resolution_order(package_id, packages, &mut visited, &mut result)?;
            }
        }

        Ok(result)
    }

    fn visit_for_resolution_order(
        &self,
        package_id: &PackageId,
        packages: &HashMap<PackageId, Vec<PackageVersion>>,
        visited: &mut HashSet<PackageId>,
        result: &mut Vec<PackageId>,
    ) -> ResolutionResult<()> {
        if visited.contains(package_id) {
            return Ok(());
        }

        visited.insert(package_id.clone());

        // Visit dependencies first (simplified - we'll use any available version for ordering)
        if let Some(versions) = packages.get(package_id) {
            if let Some(version) = versions.first() {
                let deps = version.get_applicable_dependencies(&self.context);
                for dep in deps {
                    if packages.contains_key(&dep.id) {
                        self.visit_for_resolution_order(&dep.id, packages, visited, result)?;
                    }
                }
            }
        }

        result.push(package_id.clone());
        Ok(())
    }

    fn detect_remaining_conflicts(&self, _resolution: &HashMap<PackageId, PackageVersion>) -> Vec<DependencyConflict> {
        // This would contain logic to detect any remaining conflicts after resolution
        // For now, return empty as the resolution phase should handle conflicts
        Vec::new()
    }

    #[allow(dead_code)]
    pub fn get_local_registry(&self) -> &LocalPackageRegistry {
        &self.local_registry
    }

    #[allow(dead_code)]
    pub fn get_local_registry_mut(&mut self) -> &mut LocalPackageRegistry {
        &mut self.local_registry
    }

    #[allow(dead_code)]
    pub fn get_remote_registry(&self) -> &RemotePackageRegistry {
        &self.remote_registry
    }

    #[allow(dead_code)]
    pub fn get_remote_registry_mut(&mut self) -> &mut RemotePackageRegistry {
        &mut self.remote_registry
    }

    #[allow(dead_code)]
    pub fn get_cache(&self) -> &ResolutionCache {
        &self.cache
    }

    #[allow(dead_code)]
    pub async fn clear_cache(&mut self) -> ResolutionResult<()> {
        self.cache.clear_cache().await
    }

    #[allow(dead_code)]
    pub async fn get_cache_stats(&self) -> crate::dependency::cache::CacheStats {
        self.cache.cache_stats().await
    }

    fn generate_warnings(&self, resolution: &HashMap<PackageId, PackageVersion>) -> Vec<String> {
        let mut warnings = Vec::new();

        // Check for prerelease versions
        for (package_id, package_version) in resolution {
            if !package_version.version.pre.is_empty() {
                warnings.push(format!(
                    "Using prerelease version {} for package '{}'",
                    package_version.version,
                    package_id.name
                ));
            }
        }

        // Check for security vulnerabilities (placeholder for future implementation)
        // TODO: Integrate with vulnerability databases
        
        // Check for major version conflicts (different major versions of same package)
        let mut major_versions: HashMap<String, HashSet<u64>> = HashMap::new();
        for (package_id, package_version) in resolution {
            major_versions.entry(package_id.name.clone())
                .or_default()
                .insert(package_version.version.major);
        }

        for (package_name, majors) in major_versions {
            if majors.len() > 1 {
                warnings.push(format!(
                    "Multiple major versions of '{}' in dependency tree: {}",
                    package_name,
                    majors.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(", ")
                ));
            }
        }

        warnings
    }

    #[allow(dead_code)]
    pub async fn get_package_info(&self, package_id: &PackageId) -> ResolutionResult<Vec<PackageVersion>> {
        self.discover_package_versions(package_id).await
    }

    #[allow(dead_code)]
    pub async fn update_package_cache(&mut self, package_id: &PackageId) -> ResolutionResult<()> {
        // Invalidate cache for this package
        self.cache.invalidate_package_cache(&package_id.name).await?;
        
        // Re-discover package versions
        let _versions = self.discover_package_versions(package_id).await?;
        
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_context(&mut self, context: ResolutionContext) {
        self.context = context;
    }

    #[allow(dead_code)]
    pub fn get_context(&self) -> &ResolutionContext {
        &self.context
    }

    // Enhanced dependency analysis methods
    #[allow(dead_code)]
    pub async fn analyze_package_ecosystem(&self, package_id: &PackageId) -> ResolutionResult<PackageEcosystemAnalysis> {
        let versions = self.discover_package_versions(package_id).await?;
        
        let mut analysis = PackageEcosystemAnalysis {
            package_id: package_id.clone(),
            available_versions: versions.len(),
            latest_version: None,
            dependency_count: 0,
            dependents: Vec::new(),
            security_advisories: Vec::new(),
            maintenance_status: MaintenanceStatus::Unknown,
        };
        
        if let Some(latest) = versions.iter().max_by(|a, b| a.version.cmp(&b.version)) {
            analysis.latest_version = Some(latest.version.clone());
            analysis.dependency_count = latest.dependencies.len() + latest.dev_dependencies.len();
        }
        
        // TODO: Add more sophisticated analysis
        
        Ok(analysis)
    }

    #[allow(dead_code)]
    pub async fn find_dependency_path(&self, from_package: &PackageId, to_package: &PackageId) -> ResolutionResult<Option<Vec<PackageId>>> {
        // This would find the shortest dependency path between two packages
        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        let mut parent_map: std::collections::HashMap<PackageId, PackageId> = std::collections::HashMap::new();
        
        queue.push_back(from_package.clone());
        visited.insert(from_package.clone());
        
        while let Some(current) = queue.pop_front() {
            if current == *to_package {
                // Reconstruct path
                let mut path = Vec::new();
                let mut node = to_package.clone();
                
                while node != *from_package {
                    path.push(node.clone());
                    if let Some(parent) = parent_map.get(&node) {
                        node = parent.clone();
                    } else {
                        break;
                    }
                }
                
                path.push(from_package.clone());
                path.reverse();
                return Ok(Some(path));
            }
            
            // Get dependencies of current package
            if let Ok(versions) = self.discover_package_versions(&current).await {
                if let Some(version) = versions.first() {
                    let deps = version.get_applicable_dependencies(&self.context);
                    for dep in deps {
                        if !visited.contains(&dep.id) {
                            visited.insert(dep.id.clone());
                            parent_map.insert(dep.id.clone(), current.clone());
                            queue.push_back(dep.id.clone());
                        }
                    }
                }
            }
        }
        
        Ok(None)
    }
}

// Simple edit distance implementation for finding similar package names
fn edit_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        dp[i][0] = i;
    }
    for j in 0..=len2 {
        dp[0][j] = j;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            dp[i][j] = std::cmp::min(
                std::cmp::min(dp[i - 1][j] + 1, dp[i][j - 1] + 1),
                dp[i - 1][j - 1] + cost,
            );
        }
    }

    dp[len1][len2]
}

// Additional analysis types
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PackageEcosystemAnalysis {
    pub package_id: PackageId,
    pub available_versions: usize,
    pub latest_version: Option<semver::Version>,
    pub dependency_count: usize,
    pub dependents: Vec<PackageId>,
    pub security_advisories: Vec<SecurityAdvisory>,
    pub maintenance_status: MaintenanceStatus,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SecurityAdvisory {
    pub id: String,
    pub severity: SecuritySeverity,
    pub title: String,
    pub description: String,
    pub affected_versions: semver::VersionReq,
    pub fixed_version: Option<semver::Version>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecuritySeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaintenanceStatus {
    Active,
    Maintained,
    Deprecated,
    Unmaintained,
    Unknown,
}