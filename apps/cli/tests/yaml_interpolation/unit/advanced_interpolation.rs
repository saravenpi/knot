use super::super::{test_utils::*, *};
use std::collections::HashMap;

#[cfg(test)]
mod nested_variable_tests {
    use super::*;

    #[test]
    fn test_simple_nested_variables() {
        let mut variables = HashMap::new();
        variables.insert("base_name".to_string(), "my-app".to_string());
        variables.insert("full_name".to_string(), "${base_name}-v2".to_string());
        
        let input = "name: ${full_name}";
        let expected = "name: my-app-v2";
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_deep_nested_variables() {
        let mut variables = HashMap::new();
        variables.insert("env".to_string(), "prod".to_string());
        variables.insert("region".to_string(), "us-west".to_string());
        variables.insert("cluster".to_string(), "${env}-${region}".to_string());
        variables.insert("service".to_string(), "api-${cluster}".to_string());
        variables.insert("full_service_name".to_string(), "${service}-service".to_string());
        
        let input = "service_name: ${full_service_name}";
        let expected = "service_name: api-prod-us-west-service";
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_nested_variables_with_builtin() {
        let mut variables = HashMap::new();
        variables.insert("service_name".to_string(), "${project_name}-api".to_string());
        variables.insert("full_url".to_string(), "https://${service_name}.example.com".to_string());
        
        let input = "url: ${full_url}";
        let expected = "url: https://knot-test-api.example.com";
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_multiple_nested_in_single_value() {
        let mut variables = HashMap::new();
        variables.insert("org".to_string(), "myorg".to_string());
        variables.insert("project".to_string(), "myproject".to_string());
        variables.insert("env".to_string(), "staging".to_string());
        variables.insert("prefix".to_string(), "${org}-${project}".to_string());
        variables.insert("suffix".to_string(), "${env}-v1".to_string());
        
        let input = "resource_name: ${prefix}-${suffix}";
        let expected = "resource_name: myorg-myproject-staging-v1";
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }
}

#[cfg(test)]
mod complex_interpolation_tests {
    use super::*;

    #[test]
    fn test_complex_interpolation_patterns() {
        let mut variables = HashMap::new();
        variables.insert("app_name".to_string(), "awesome-app".to_string());
        variables.insert("version_major".to_string(), "2".to_string());
        variables.insert("version_minor".to_string(), "1".to_string());
        variables.insert("version_patch".to_string(), "3".to_string());
        variables.insert("build_number".to_string(), "42".to_string());
        
        let input = r#"
name: ${app_name}
version: ${version_major}.${version_minor}.${version_patch}
build: ${version_major}.${version_minor}.${version_patch}+build.${build_number}
docker_tag: ${app_name}:${version_major}.${version_minor}.${version_patch}
full_name: ${app_name}_v${version_major}_${version_minor}_${version_patch}
"#;
        
        let expected = r#"
name: awesome-app
version: 2.1.3
build: 2.1.3+build.42
docker_tag: awesome-app:2.1.3
full_name: awesome-app_v2_1_3
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_interpolation_in_file_paths() {
        let mut variables = HashMap::new();
        variables.insert("project_root".to_string(), "/home/user/projects".to_string());
        variables.insert("project_name".to_string(), "my-app".to_string());
        variables.insert("build_dir".to_string(), "${project_root}/${project_name}/build".to_string());
        variables.insert("dist_dir".to_string(), "${build_dir}/dist".to_string());
        
        let input = r#"
paths:
  src: "${project_root}/${project_name}/src"
  build: "${build_dir}"
  dist: "${dist_dir}"
  config: "${project_root}/${project_name}/config.yml"
"#;
        
        let expected = r#"
paths:
  src: "/home/user/projects/my-app/src"
  build: "/home/user/projects/my-app/build"
  dist: "/home/user/projects/my-app/build/dist"
  config: "/home/user/projects/my-app/config.yml"
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_interpolation_with_special_formats() {
        let mut variables = HashMap::new();
        variables.insert("service_name".to_string(), "user-service".to_string());
        variables.insert("namespace".to_string(), "production".to_string());
        variables.insert("port".to_string(), "8080".to_string());
        variables.insert("protocol".to_string(), "https".to_string());
        
        let input = r#"
kubernetes:
  service: "${service_name}.${namespace}.svc.cluster.local:${port}"
  url: "${protocol}://${service_name}.${namespace}.example.com"
  labels:
    app: "${service_name}"
    namespace: "${namespace}"
    version: "v1.0.0"
database:
  connection_string: "${protocol}://db-${service_name}.${namespace}:5432/app"
"#;
        
        let expected = r#"
kubernetes:
  service: "user-service.production.svc.cluster.local:8080"
  url: "https://user-service.production.example.com"
  labels:
    app: "user-service"
    namespace: "production"
    version: "v1.0.0"
database:
  connection_string: "https://db-user-service.production:5432/app"
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_interpolation_in_json_like_structures() {
        let mut variables = HashMap::new();
        variables.insert("api_key".to_string(), "abc123".to_string());
        variables.insert("base_url".to_string(), "https://api.example.com".to_string());
        variables.insert("version".to_string(), "v1".to_string());
        
        let input = r#"
api_config: |
  {
    "apiKey": "${api_key}",
    "baseUrl": "${base_url}/${version}",
    "timeout": 30000,
    "endpoints": {
      "users": "${base_url}/${version}/users",
      "posts": "${base_url}/${version}/posts"
    }
  }
"#;
        
        let expected = r#"
api_config: |
  {
    "apiKey": "abc123",
    "baseUrl": "https://api.example.com/v1",
    "timeout": 30000,
    "endpoints": {
      "users": "https://api.example.com/v1/users",
      "posts": "https://api.example.com/v1/posts"
    }
  }
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }
}

#[cfg(test)]
mod environment_fallback_tests {
    use super::*;
    use std::env;

    #[test]
    fn test_environment_variable_fallback() {
        env::set_var("TEST_ENV_VAR", "from_environment");
        
        let variables = HashMap::new(); // No user-defined variables
        let interpolator = YamlInterpolator::new().with_env_fallbacks(true);
        
        let input = "value: ${TEST_ENV_VAR}";
        let result = interpolator.interpolate(input, &variables).unwrap();
        
        assert_eq!(result, "value: from_environment");
        
        env::remove_var("TEST_ENV_VAR");
    }

    #[test]
    fn test_user_variables_override_env() {
        env::set_var("TEST_OVERRIDE", "env_value");
        
        let mut variables = HashMap::new();
        variables.insert("TEST_OVERRIDE".to_string(), "user_value".to_string());
        
        let interpolator = YamlInterpolator::new().with_env_fallbacks(true);
        
        let input = "value: ${TEST_OVERRIDE}";
        let result = interpolator.interpolate(input, &variables).unwrap();
        
        assert_eq!(result, "value: user_value");
        
        env::remove_var("TEST_OVERRIDE");
    }

    #[test]
    fn test_env_fallback_with_nested_variables() {
        env::set_var("APP_ENV", "production");
        env::set_var("APP_PORT", "8080");
        
        let mut variables = HashMap::new();
        variables.insert("app_name".to_string(), "myapp".to_string());
        variables.insert("service_url".to_string(), "${app_name}-${APP_ENV}:${APP_PORT}".to_string());
        
        let interpolator = YamlInterpolator::new().with_env_fallbacks(true);
        
        let input = "service: ${service_url}";
        let result = interpolator.interpolate(input, &variables).unwrap();
        
        assert_eq!(result, "service: myapp-production:8080");
        
        env::remove_var("APP_ENV");
        env::remove_var("APP_PORT");
    }

    #[test]
    fn test_undefined_env_var_without_fallback_fails() {
        let variables = HashMap::new();
        let interpolator = YamlInterpolator::new(); // No env fallbacks
        
        let input = "value: ${UNDEFINED_ENV_VAR}";
        let result = interpolator.interpolate(input, &variables);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Undefined variable"));
    }

    #[test]
    fn test_undefined_env_var_with_fallback_fails() {
        let variables = HashMap::new();
        let interpolator = YamlInterpolator::new().with_env_fallbacks(true);
        
        let input = "value: ${DEFINITELY_UNDEFINED_ENV_VAR_12345}";
        let result = interpolator.interpolate(input, &variables);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Undefined variable"));
    }
}

#[cfg(test)]
mod interpolation_in_keys_tests {
    use super::*;

    #[test]
    fn test_variable_interpolation_in_yaml_keys() {
        let mut variables = HashMap::new();
        variables.insert("env".to_string(), "production".to_string());
        variables.insert("service".to_string(), "api".to_string());
        
        let input = r#"
services:
  ${service}_${env}:
    image: "myapp:latest"
    port: 8080
environments:
  ${env}:
    debug: false
    log_level: "info"
"#;
        
        let expected = r#"
services:
  api_production:
    image: "myapp:latest"
    port: 8080
environments:
  production:
    debug: false
    log_level: "info"
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_nested_keys_with_variables() {
        let mut variables = HashMap::new();
        variables.insert("region".to_string(), "us-west-2".to_string());
        variables.insert("az".to_string(), "a".to_string());
        variables.insert("instance_type".to_string(), "t3.micro".to_string());
        
        let input = r#"
aws:
  regions:
    ${region}:
      availability_zones:
        ${region}${az}:
          instance_type: ${instance_type}
          spot_pricing: true
"#;
        
        let expected = r#"
aws:
  regions:
    us-west-2:
      availability_zones:
        us-west-2a:
          instance_type: t3.micro
          spot_pricing: true
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }

    #[test]
    fn test_dynamic_configuration_keys() {
        let mut variables = HashMap::new();
        variables.insert("config_env".to_string(), "staging".to_string());
        variables.insert("feature_flag".to_string(), "new_ui".to_string());
        
        let input = r#"
configuration:
  ${config_env}_config:
    database_url: "postgres://localhost/myapp_${config_env}"
    cache_enabled: true
  
feature_flags:
  ${feature_flag}_enabled: true
  ${feature_flag}_rollout_percent: 50
"#;
        
        let expected = r#"
configuration:
  staging_config:
    database_url: "postgres://localhost/myapp_staging"
    cache_enabled: true
  
feature_flags:
  new_ui_enabled: true
  new_ui_rollout_percent: 50
"#;
        
        assert_interpolation_equals(input, expected, variables).unwrap();
    }
}