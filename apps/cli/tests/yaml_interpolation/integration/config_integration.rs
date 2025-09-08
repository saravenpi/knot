use super::super::{test_utils::*, *};
use crate::config::{KnotConfig, PackageConfig, AppConfig, AppDependencies, TsAlias};
use std::collections::HashMap;

/// Extended interpolator that works with real Knot config types
pub struct KnotConfigInterpolator {
    interpolator: YamlInterpolator,
}

impl KnotConfigInterpolator {
    pub fn new() -> Self {
        let mut interpolator = YamlInterpolator::new();
        // Add common built-in variables for Knot configs
        interpolator.add_built_in_var("knot_version", "1.0.0");
        interpolator.add_built_in_var("node_version", "18.0.0");
        interpolator.add_built_in_var("typescript_version", "5.0.0");
        
        Self { interpolator }
    }

    pub fn parse_knot_config(&self, yaml_content: &str) -> anyhow::Result<KnotConfig> {
        self.interpolator.parse_yaml_with_interpolation(yaml_content)
    }

    pub fn parse_package_config(&self, yaml_content: &str) -> anyhow::Result<PackageConfig> {
        self.interpolator.parse_yaml_with_interpolation(yaml_content)
    }

    pub fn parse_app_config(&self, yaml_content: &str) -> anyhow::Result<AppConfig> {
        self.interpolator.parse_yaml_with_interpolation(yaml_content)
    }
}

#[cfg(test)]
mod knot_config_tests {
    use super::*;

    #[test]
    fn test_knot_config_basic_interpolation() {
        let yaml_content = r#"
variables:
  project_name: "awesome-monorepo"
  project_description: "An awesome monorepo project"
  
name: ${project_name}
description: ${project_description}

scripts:
  build: "echo Building ${project_name}"
  test: "echo Testing ${project_name}"
  deploy: "echo Deploying ${project_name} to production"

apps:
  frontend:
    description: "Frontend app for ${project_name}"
    packages: ["ui-components", "utils"]
  backend:
    description: "Backend API for ${project_name}"
    packages: ["types", "database"]
"#;

        let interpolator = KnotConfigInterpolator::new();
        let config = interpolator.parse_knot_config(yaml_content).unwrap();

        assert_eq!(config.name, "awesome-monorepo");
        assert_eq!(config.description, Some("An awesome monorepo project".to_string()));

        let scripts = config.scripts.unwrap();
        assert_eq!(scripts.get("build").unwrap(), "echo Building awesome-monorepo");
        assert_eq!(scripts.get("test").unwrap(), "echo Testing awesome-monorepo");
        assert_eq!(scripts.get("deploy").unwrap(), "echo Deploying awesome-monorepo to production");

        let apps = config.apps.unwrap();
        assert!(apps.contains_key("frontend"));
        assert!(apps.contains_key("backend"));

        // Test that apps contain the interpolated descriptions (this would require extending the current config structures)
        // For now, we verify the structure is correct
        match apps.get("frontend").unwrap() {
            AppDependencies::List(packages) => {
                assert_eq!(packages, &vec!["ui-components".to_string(), "utils".to_string()]);
            }
            AppDependencies::Object { packages, .. } => {
                assert_eq!(packages.as_ref().unwrap(), &vec!["ui-components".to_string(), "utils".to_string()]);
            }
        }
    }

    #[test]
    fn test_knot_config_with_ts_alias_interpolation() {
        let yaml_content = r#"
variables:
  project_name: "my-workspace"
  alias_symbol: "@"

name: ${project_name}
tsAlias: ${alias_symbol}

apps:
  web:
    tsAlias: true
    packages: ["types", "utils"]
  api:
    tsAlias: ${alias_symbol}
    packages: ["database", "types"]
"#;

        let interpolator = KnotConfigInterpolator::new();
        let config = interpolator.parse_knot_config(yaml_content).unwrap();

        assert_eq!(config.name, "my-workspace");
        
        match config.ts_alias.unwrap() {
            TsAlias::String(alias) => assert_eq!(alias, "@"),
            _ => panic!("Expected string alias"),
        }

        let apps = config.apps.unwrap();
        
        // Check web app has boolean true alias
        if let AppDependencies::Object { ts_alias, .. } = apps.get("web").unwrap() {
            match ts_alias.as_ref().unwrap() {
                TsAlias::Boolean(true) => {}, // Expected
                _ => panic!("Expected boolean true alias"),
            }
        }

        // Check api app has string alias
        if let AppDependencies::Object { ts_alias, .. } = apps.get("api").unwrap() {
            match ts_alias.as_ref().unwrap() {
                TsAlias::String(alias) => assert_eq!(alias, "@"),
                _ => panic!("Expected string alias"),
            }
        }
    }

    #[test]
    fn test_knot_config_complex_script_interpolation() {
        let yaml_content = r#"
variables:
  project_name: "knot-workspace"
  build_tool: "turbo"
  test_tool: "jest"
  workspace_cmd: "bun run"
  docker_registry: "registry.example.com"

name: ${project_name}

scripts:
  install: "${workspace_cmd} install"
  build: "${build_tool} run build"
  test: "${test_tool} --passWithNoTests"
  lint: "${build_tool} run lint"
  clean: "${build_tool} run clean"
  docker-build: "docker build -t ${docker_registry}/${project_name}:latest ."
  docker-push: "docker push ${docker_registry}/${project_name}:latest"
  deploy: "${workspace_cmd} docker-build && ${workspace_cmd} docker-push"
"#;

        let interpolator = KnotConfigInterpolator::new();
        let config = interpolator.parse_knot_config(yaml_content).unwrap();

        let scripts = config.scripts.unwrap();
        assert_eq!(scripts.get("install").unwrap(), "bun run install");
        assert_eq!(scripts.get("build").unwrap(), "turbo run build");
        assert_eq!(scripts.get("test").unwrap(), "jest --passWithNoTests");
        assert_eq!(scripts.get("docker-build").unwrap(), "docker build -t registry.example.com/knot-workspace:latest .");
        assert_eq!(scripts.get("docker-push").unwrap(), "docker push registry.example.com/knot-workspace:latest");
        assert_eq!(scripts.get("deploy").unwrap(), "bun run docker-build && bun run docker-push");
    }
}

#[cfg(test)]
mod package_config_tests {
    use super::*;

    #[test]
    fn test_package_config_interpolation() {
        let yaml_content = r#"
variables:
  pkg_name: "ui-components"
  pkg_version: "2.1.0"
  author_name: "saravenpi"
  team_name: "frontend"
  repo_org: "myorg"

name: ${pkg_name}
version: ${pkg_version}
team: ${team_name}
author: ${author_name}
description: "UI component library for ${pkg_name}"
repository: "https://github.com/${repo_org}/${pkg_name}"
license: "MIT"

keywords:
  - "ui"
  - "components"
  - "${pkg_name}"
  - "react"

tags:
  - "ui-lib"
  - "${team_name}-package"

scripts:
  build: "tsc && vite build"
  test: "vitest run"
  storybook: "storybook dev -p 6006"
  publish: "npm publish --registry https://registry.${repo_org}.com"

dependencies:
  - "react"
  - "react-dom"
  - "@types/react"

dev_dependencies:
  - "typescript"
  - "vite"
  - "vitest"
  - "storybook"

exports:
  ".": "./dist/index.js"
  "./components": "./dist/components/index.js"
  "./styles": "./dist/styles/index.css"
"#;

        let interpolator = KnotConfigInterpolator::new();
        let config = interpolator.parse_package_config(yaml_content).unwrap();

        assert_eq!(config.name, "ui-components");
        assert_eq!(config.version, "2.1.0");
        assert_eq!(config.team, Some("frontend".to_string()));
        assert_eq!(config.author, Some("saravenpi".to_string()));
        assert_eq!(config.description, Some("UI component library for ui-components".to_string()));
        assert_eq!(config.repository, Some("https://github.com/myorg/ui-components".to_string()));

        let keywords = config.keywords.unwrap();
        assert!(keywords.contains(&"ui-components".to_string()));

        let tags = config.tags.unwrap();
        assert!(tags.contains(&"frontend-package".to_string()));

        let scripts = config.scripts.unwrap();
        assert_eq!(scripts.get("publish").unwrap(), "npm publish --registry https://registry.myorg.com");
    }

    #[test]
    fn test_package_config_with_nested_variables() {
        let yaml_content = r#"
variables:
  base_name: "data"
  namespace: "api"
  team: "backend"
  full_name: "${namespace}-${base_name}"
  docker_image: "registry.example.com/${team}/${full_name}"

name: ${full_name}
version: 1.0.0
team: ${team}
description: "Data access layer for ${namespace} services"

scripts:
  build: "tsc"
  test: "jest"
  docker-build: "docker build -t ${docker_image}:latest ."
  docker-test: "docker run --rm ${docker_image}:latest npm test"

dependencies:
  - "prisma"
  - "postgresql"

features:
  - "database-${base_name}"
  - "${team}-integration"
  - "${full_name}-caching"
"#;

        let interpolator = KnotConfigInterpolator::new();
        let config = interpolator.parse_package_config(yaml_content).unwrap();

        assert_eq!(config.name, "api-data");
        assert_eq!(config.team, Some("backend".to_string()));
        assert_eq!(config.description, Some("Data access layer for api services".to_string()));

        let scripts = config.scripts.unwrap();
        assert_eq!(scripts.get("docker-build").unwrap(), "docker build -t registry.example.com/backend/api-data:latest .");
        assert_eq!(scripts.get("docker-test").unwrap(), "docker run --rm registry.example.com/backend/api-data:latest npm test");

        let features = config.features.unwrap();
        assert!(features.contains(&"database-data".to_string()));
        assert!(features.contains(&"backend-integration".to_string()));
        assert!(features.contains(&"api-data-caching".to_string()));
    }
}

#[cfg(test)]
mod app_config_tests {
    use super::*;

    #[test]
    fn test_app_config_interpolation() {
        let yaml_content = r#"
variables:
  app_name: "web-dashboard"
  app_description: "Web dashboard application"
  alias_symbol: "#"
  package1: "ui-components"
  package2: "data-client"
  package3: "utils"

name: ${app_name}
description: ${app_description}
tsAlias: ${alias_symbol}

packages:
  - ${package1}
  - ${package2}
  - ${package3}

scripts:
  dev: "vite dev"
  build: "vite build"
  preview: "vite preview"
  test: "vitest run"
  test-ui: "vitest --ui"
  storybook: "storybook dev -p 6006"
  analyze: "vite-bundle-analyzer"
"#;

        let interpolator = KnotConfigInterpolator::new();
        let config = interpolator.parse_app_config(yaml_content).unwrap();

        assert_eq!(config.name, "web-dashboard");
        assert_eq!(config.description, Some("Web dashboard application".to_string()));
        
        match config.ts_alias.unwrap() {
            TsAlias::String(alias) => assert_eq!(alias, "#"),
            _ => panic!("Expected string alias"),
        }

        let packages = config.packages.unwrap();
        assert_eq!(packages, vec!["ui-components", "data-client", "utils"]);

        let scripts = config.scripts.unwrap();
        assert_eq!(scripts.len(), 7);
        assert!(scripts.contains_key("dev"));
        assert!(scripts.contains_key("build"));
    }

    #[test]
    fn test_app_config_with_environment_specific_variables() {
        let yaml_content = r#"
variables:
  app_name: "api-server"
  env: "development"
  port: "3000"
  host: "localhost"
  database_url: "postgresql://${host}:5432/${app_name}_${env}"
  log_level: "debug"

name: ${app_name}
description: "API server for ${env} environment"

packages:
  - "database"
  - "auth"
  - "api-types"

scripts:
  dev: "nodemon --exec 'node -r ts-node/register src/index.ts'"
  build: "tsc"
  start: "node dist/index.js"
  test: "jest"
  migrate: "prisma migrate dev"
  seed: "ts-node src/seed.ts"
  docker: "docker run -p ${port}:${port} ${app_name}:latest"
"#;

        let interpolator = KnotConfigInterpolator::new();
        let config = interpolator.parse_app_config(yaml_content).unwrap();

        assert_eq!(config.name, "api-server");
        assert_eq!(config.description, Some("API server for development environment".to_string()));

        let scripts = config.scripts.unwrap();
        assert_eq!(scripts.get("docker").unwrap(), "docker run -p 3000:3000 api-server:latest");
    }
}

#[cfg(test)]
mod variable_inheritance_tests {
    use super::*;

    #[test]
    fn test_builtin_variables_in_real_configs() {
        let yaml_content = r#"
name: ${project_name}
description: "Built with Knot v${knot_version}"

scripts:
  info: "echo 'Project: ${project_name}, Node: ${node_version}, TypeScript: ${typescript_version}'"
  version: "echo 'Knot version: ${knot_version}'"
"#;

        let interpolator = KnotConfigInterpolator::new();
        let config = interpolator.parse_knot_config(yaml_content).unwrap();

        assert_eq!(config.name, "knot-test"); // Built-in project_name
        assert_eq!(config.description, Some("Built with Knot v1.0.0".to_string()));

        let scripts = config.scripts.unwrap();
        assert_eq!(scripts.get("info").unwrap(), "echo 'Project: knot-test, Node: 18.0.0, TypeScript: 5.0.0'");
        assert_eq!(scripts.get("version").unwrap(), "echo 'Knot version: 1.0.0'");
    }

    #[test]
    fn test_variable_override_priority() {
        let yaml_content = r#"
variables:
  project_name: "custom-project-name"  # Override built-in
  knot_version: "2.0.0"               # Override built-in
  custom_var: "custom-value"          # New variable

name: ${project_name}
description: "Built with Knot v${knot_version}"

scripts:
  custom: "echo ${custom_var}"
  builtin: "echo Node: ${node_version}"  # This built-in should still work
"#;

        let interpolator = KnotConfigInterpolator::new();
        let config = interpolator.parse_knot_config(yaml_content).unwrap();

        assert_eq!(config.name, "custom-project-name");
        assert_eq!(config.description, Some("Built with Knot v2.0.0".to_string()));

        let scripts = config.scripts.unwrap();
        assert_eq!(scripts.get("custom").unwrap(), "echo custom-value");
        assert_eq!(scripts.get("builtin").unwrap(), "echo Node: 18.0.0");
    }
}

#[cfg(test)]
mod backwards_compatibility_tests {
    use super::*;

    #[test]
    fn test_config_without_variables_section_still_works() {
        let yaml_content = r#"
name: static-project
description: "A static configuration without variables"

scripts:
  build: "npm run build"
  test: "npm test"

apps:
  frontend:
    packages: ["ui", "utils"]
"#;

        let interpolator = KnotConfigInterpolator::new();
        let config = interpolator.parse_knot_config(yaml_content).unwrap();

        assert_eq!(config.name, "static-project");
        assert_eq!(config.description, Some("A static configuration without variables".to_string()));
        
        let scripts = config.scripts.unwrap();
        assert_eq!(scripts.get("build").unwrap(), "npm run build");
    }

    #[test]
    fn test_mixed_interpolated_and_static_content() {
        let yaml_content = r#"
variables:
  app_name: "hybrid-app"

name: ${app_name}
version: "1.0.0"  # Static value
description: "App ${app_name} with static version"

scripts:
  build: "npm run build"  # Static
  test: "npm test -- --testNamePattern=${app_name}"  # Mixed
  deploy: "deploy ${app_name}"  # Interpolated
"#;

        let interpolator = KnotConfigInterpolator::new();
        let config = interpolator.parse_knot_config(yaml_content).unwrap();

        assert_eq!(config.name, "hybrid-app");
        assert_eq!(config.description, Some("App hybrid-app with static version".to_string()));
        
        let scripts = config.scripts.unwrap();
        assert_eq!(scripts.get("build").unwrap(), "npm run build");
        assert_eq!(scripts.get("test").unwrap(), "npm test -- --testNamePattern=hybrid-app");
        assert_eq!(scripts.get("deploy").unwrap(), "deploy hybrid-app");
    }

    #[test]
    fn test_config_validation_still_works_with_interpolation() {
        let yaml_content = r#"
variables:
  project_name: "valid-project"

name: ${project_name}
version: 1.0.0

scripts:
  build: "echo Building ${project_name}"
"#;

        let interpolator = KnotConfigInterpolator::new();
        let config = interpolator.parse_knot_config(yaml_content).unwrap();
        
        // The config should still pass validation
        assert!(config.validate().is_ok());
        assert_eq!(config.name, "valid-project");
    }
}