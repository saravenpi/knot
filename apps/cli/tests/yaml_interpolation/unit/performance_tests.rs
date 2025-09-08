use super::super::{test_utils::*, *};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance test utilities
struct PerformanceTest {
    interpolator: YamlInterpolator,
}

impl PerformanceTest {
    fn new() -> Self {
        Self {
            interpolator: YamlInterpolator::new(),
        }
    }

    /// Time a function execution and return (result, duration)
    fn time_execution<F, R>(&self, func: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = func();
        let duration = start.elapsed();
        (result, duration)
    }

    /// Generate a large number of variables for testing
    fn generate_variables(&self, count: usize) -> HashMap<String, String> {
        let mut variables = HashMap::new();
        for i in 0..count {
            variables.insert(
                format!("var_{}", i),
                format!("value_for_variable_{}", i),
            );
        }
        variables
    }

    /// Generate nested variables for testing complex interpolation
    fn generate_nested_variables(&self, depth: usize) -> HashMap<String, String> {
        let mut variables = HashMap::new();
        
        // Base variable
        variables.insert("base".to_string(), "base_value".to_string());
        
        // Create nested chain
        for i in 0..depth {
            if i == 0 {
                variables.insert(format!("level_{}", i), "${base}".to_string());
            } else {
                variables.insert(format!("level_{}", i), format!("${{level_{}}}", i - 1));
            }
        }
        
        variables
    }

    /// Generate a large YAML configuration for testing
    fn generate_large_yaml(&self, num_sections: usize, vars_per_section: usize) -> String {
        let mut yaml = String::new();
        yaml.push_str("variables:\n");
        
        // Generate variables section
        for i in 0..num_sections * vars_per_section {
            yaml.push_str(&format!("  var_{}: \"value_{}\"\n", i, i));
        }
        
        yaml.push_str("\n");
        
        // Generate config sections
        for section in 0..num_sections {
            yaml.push_str(&format!("section_{}:\n", section));
            yaml.push_str("  metadata:\n");
            yaml.push_str(&format!("    name: \"Section ${{{}_name}}\"\n", section));
            yaml.push_str(&format!("    id: ${{var_{}}}\n", section));
            
            yaml.push_str("  config:\n");
            for var in 0..vars_per_section {
                let var_idx = section * vars_per_section + var;
                yaml.push_str(&format!("    field_{}: ${{var_{}}}\n", var, var_idx));
            }
            
            yaml.push_str("  scripts:\n");
            for script in 0..5 {
                let var_idx = section * vars_per_section + (script % vars_per_section);
                yaml.push_str(&format!("    script_{}: \"echo Processing ${{var_{}}} in section {}\"\n", 
                                     script, var_idx, section));
            }
            yaml.push_str("\n");
        }
        
        yaml
    }
}

#[cfg(test)]
mod large_config_tests {
    use super::*;

    #[test]
    fn test_large_number_of_variables() {
        let perf_test = PerformanceTest::new();
        let variables = perf_test.generate_variables(1000);
        
        let input = (0..1000)
            .map(|i| format!("field_{}: ${{var_{}}}", i, i))
            .collect::<Vec<_>>()
            .join("\n");
        
        let (result, duration) = perf_test.time_execution(|| {
            perf_test.interpolator.interpolate(&input, &variables)
        });
        
        assert!(result.is_ok(), "Large variable interpolation should succeed");
        println!("1000 variables interpolation took: {:?}", duration);
        
        // Performance assertion - should complete within reasonable time
        assert!(duration < Duration::from_millis(1000), 
                "Interpolation with 1000 variables took too long: {:?}", duration);
    }

    #[test]
    fn test_very_large_config_file() {
        let perf_test = PerformanceTest::new();
        let yaml_content = perf_test.generate_large_yaml(100, 20); // 100 sections, 20 vars each
        
        println!("Generated YAML size: {} bytes", yaml_content.len());
        
        let (result, duration) = perf_test.time_execution(|| {
            perf_test.interpolator.parse_yaml_with_interpolation::<serde_yaml::Value>(&yaml_content)
        });
        
        assert!(result.is_ok(), "Large YAML parsing should succeed");
        println!("Large YAML ({}KB) parsing took: {:?}", yaml_content.len() / 1024, duration);
        
        // Should complete within reasonable time even for large files
        assert!(duration < Duration::from_secs(5), 
                "Large YAML parsing took too long: {:?}", duration);
    }

    #[test]
    fn test_memory_usage_with_large_configs() {
        let perf_test = PerformanceTest::new();
        
        // Test with progressively larger configs to check for memory leaks
        for multiplier in [10, 50, 100, 200] {
            let variables = perf_test.generate_variables(multiplier * 10);
            let yaml_content = perf_test.generate_large_yaml(multiplier, 10);
            
            let (result, duration) = perf_test.time_execution(|| {
                perf_test.interpolator.parse_yaml_with_interpolation::<serde_yaml::Value>(&yaml_content)
            });
            
            assert!(result.is_ok(), "Config parsing should succeed for {} multiplier", multiplier);
            println!("Config size {}x took: {:?}", multiplier, duration);
            
            // Memory usage should remain reasonable (not exponential growth)
            // This is a basic test - in production you'd use proper memory profiling
            assert!(duration < Duration::from_secs(10), 
                    "Config parsing took too long for size {}x: {:?}", multiplier, duration);
        }
    }

    #[test]
    fn test_repeated_interpolation_performance() {
        let perf_test = PerformanceTest::new();
        let variables = perf_test.generate_variables(100);
        
        let input = r#"
name: ${var_0}
description: "Config with ${var_1} and ${var_2}"
version: "${var_3}.${var_4}.${var_5}"
scripts:
  build: "echo ${var_6}"
  test: "echo ${var_7}"
  deploy: "echo ${var_8} ${var_9}"
"#;
        
        let iterations = 1000;
        let (results, duration) = perf_test.time_execution(|| {
            (0..iterations)
                .map(|_| perf_test.interpolator.interpolate(input, &variables))
                .collect::<Vec<_>>()
        });
        
        // All iterations should succeed
        for (i, result) in results.iter().enumerate() {
            assert!(result.is_ok(), "Iteration {} failed: {:?}", i, result);
        }
        
        let avg_duration = duration / iterations;
        println!("{} iterations took: {:?} (avg: {:?} per iteration)", 
                 iterations, duration, avg_duration);
        
        // Average per-iteration should be very fast
        assert!(avg_duration < Duration::from_millis(1), 
                "Average interpolation time too slow: {:?}", avg_duration);
    }
}

#[cfg(test)]
mod complex_interpolation_performance_tests {
    use super::*;

    #[test]
    fn test_deep_variable_nesting_performance() {
        let perf_test = PerformanceTest::new();
        let variables = perf_test.generate_nested_variables(50); // 50 levels deep
        
        let input = "final_value: ${level_49}";
        
        let (result, duration) = perf_test.time_execution(|| {
            perf_test.interpolator.interpolate(input, &variables)
        });
        
        assert!(result.is_ok(), "Deep nesting interpolation should succeed");
        assert!(result.unwrap().contains("base_value"));
        
        println!("50-level deep nesting took: {:?}", duration);
        assert!(duration < Duration::from_millis(100), 
                "Deep nesting took too long: {:?}", duration);
    }

    #[test]
    fn test_many_variables_single_line_performance() {
        let perf_test = PerformanceTest::new();
        let variables = perf_test.generate_variables(500);
        
        // Create a single line with 500 variable interpolations
        let interpolations = (0..500)
            .map(|i| format!("${{var_{}}}", i))
            .collect::<Vec<_>>()
            .join("-");
        
        let input = format!("value: {}", interpolations);
        
        let (result, duration) = perf_test.time_execution(|| {
            perf_test.interpolator.interpolate(&input, &variables)
        });
        
        assert!(result.is_ok(), "Many variables in single line should succeed");
        println!("500 variables in single line took: {:?}", duration);
        assert!(duration < Duration::from_millis(500), 
                "Too many variables interpolation took too long: {:?}", duration);
    }

    #[test]
    fn test_complex_nested_structure_performance() {
        let perf_test = PerformanceTest::new();
        
        // Create complex nested variables
        let mut variables = HashMap::new();
        variables.insert("app".to_string(), "myapp".to_string());
        variables.insert("env".to_string(), "prod".to_string());
        variables.insert("region".to_string(), "us-west".to_string());
        variables.insert("version".to_string(), "1.0.0".to_string());
        
        // Create multiple levels of nesting
        for level in 1..=10 {
            for i in 0..20 {
                let key = format!("level{}_var{}", level, i);
                let value = if level == 1 {
                    format!("${{{}}}-{}-{}", 
                           ["app", "env", "region", "version"][i % 4], level, i)
                } else {
                    format!("${{level{}_var{}}}-{}", level - 1, i % 20, level)
                };
                variables.insert(key, value);
            }
        }
        
        // Create complex YAML with nested interpolations
        let mut input = String::new();
        input.push_str("complex_config:\n");
        for level in 1..=10 {
            input.push_str(&format!("  level_{}:\n", level));
            for i in 0..10 {
                input.push_str(&format!("    field_{}: ${{level{}_var{}}}\n", i, level, i));
            }
        }
        
        let (result, duration) = perf_test.time_execution(|| {
            perf_test.interpolator.interpolate(&input, &variables)
        });
        
        assert!(result.is_ok(), "Complex nested interpolation should succeed");
        println!("Complex nested structure took: {:?}", duration);
        assert!(duration < Duration::from_secs(1), 
                "Complex nested interpolation took too long: {:?}", duration);
    }

    #[test]
    fn test_circular_dependency_detection_performance() {
        let perf_test = PerformanceTest::new();
        
        // Create a large number of variables with circular dependencies
        let mut variables = HashMap::new();
        for i in 0..100 {
            let next = (i + 1) % 100;
            variables.insert(format!("var_{}", i), format!("${{var_{}}}", next));
        }
        
        let input = "value: ${var_0}";
        
        let (result, duration) = perf_test.time_execution(|| {
            perf_test.interpolator.interpolate(input, &variables)
        });
        
        // Should fail due to circular dependency
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Circular dependency"));
        
        println!("Circular dependency detection (100 vars) took: {:?}", duration);
        // Should detect circular dependency quickly
        assert!(duration < Duration::from_millis(100), 
                "Circular dependency detection took too long: {:?}", duration);
    }
}

#[cfg(test)]
mod scalability_tests {
    use super::*;

    #[test]
    fn test_interpolation_scalability() {
        let perf_test = PerformanceTest::new();
        
        // Test with different scales to check O(n) behavior
        let scales = [10, 100, 500, 1000];
        let mut results = Vec::new();
        
        for &scale in &scales {
            let variables = perf_test.generate_variables(scale);
            let input = (0..scale)
                .map(|i| format!("field_{}: ${{var_{}}}", i, i))
                .collect::<Vec<_>>()
                .join("\n");
            
            let (result, duration) = perf_test.time_execution(|| {
                perf_test.interpolator.interpolate(&input, &variables)
            });
            
            assert!(result.is_ok(), "Scale {} should succeed", scale);
            results.push((scale, duration));
            println!("Scale {}: {:?}", scale, duration);
        }
        
        // Check that performance scales reasonably (not exponentially)
        for i in 1..results.len() {
            let (prev_scale, prev_duration) = results[i - 1];
            let (curr_scale, curr_duration) = results[i];
            
            let scale_ratio = curr_scale as f64 / prev_scale as f64;
            let time_ratio = curr_duration.as_nanos() as f64 / prev_duration.as_nanos() as f64;
            
            println!("Scale {}x to {}x: time ratio {:.2}", 
                     prev_scale, curr_scale, time_ratio);
            
            // Time should scale roughly linearly, not exponentially
            // Allow some overhead but shouldn't be more than quadratic
            assert!(time_ratio < scale_ratio * scale_ratio, 
                    "Performance scaling is worse than quadratic");
        }
    }

    #[test]
    fn test_concurrent_interpolation_safety() {
        use std::thread;
        use std::sync::Arc;
        
        let perf_test = Arc::new(PerformanceTest::new());
        let variables = Arc::new(perf_test.generate_variables(100));
        
        let input = r#"
concurrent_test:
  field1: ${var_1}
  field2: ${var_2}
  field3: ${var_3}
"#;
        
        let handles: Vec<_> = (0..10).map(|thread_id| {
            let perf_test = Arc::clone(&perf_test);
            let variables = Arc::clone(&variables);
            let input = input.to_string();
            
            thread::spawn(move || {
                let (result, duration) = perf_test.time_execution(|| {
                    perf_test.interpolator.interpolate(&input, &variables)
                });
                (thread_id, result, duration)
            })
        }).collect();
        
        let results: Vec<_> = handles.into_iter()
            .map(|h| h.join().unwrap())
            .collect();
        
        // All threads should succeed
        for (thread_id, result, duration) in results {
            assert!(result.is_ok(), "Thread {} failed: {:?}", thread_id, result);
            println!("Thread {}: {:?}", thread_id, duration);
        }
    }

    #[test]
    fn test_memory_efficiency() {
        let perf_test = PerformanceTest::new();
        
        // Create a test that would use a lot of memory if inefficient
        let large_value = "x".repeat(1000);
        let mut variables = HashMap::new();
        
        for i in 0..1000 {
            variables.insert(format!("var_{}", i), large_value.clone());
        }
        
        let input = (0..1000)
            .map(|i| format!("field_{}: ${{var_{}}}", i, i))
            .collect::<Vec<_>>()
            .join("\n");
        
        let (result, duration) = perf_test.time_execution(|| {
            perf_test.interpolator.interpolate(&input, &variables)
        });
        
        assert!(result.is_ok(), "Memory efficiency test should succeed");
        
        let result_content = result.unwrap();
        println!("Memory efficiency test - input: {}KB, output: {}KB, time: {:?}",
                 input.len() / 1024,
                 result_content.len() / 1024,
                 duration);
        
        // Should complete in reasonable time even with large content
        assert!(duration < Duration::from_secs(2), 
                "Memory efficiency test took too long: {:?}", duration);
    }
}

#[cfg(test)]
mod benchmarks {
    use super::*;

    /// Benchmark utility for consistent measurements
    fn benchmark<F>(name: &str, iterations: usize, func: F) -> Duration 
    where
        F: Fn(),
    {
        // Warmup
        for _ in 0..10 {
            func();
        }
        
        let start = Instant::now();
        for _ in 0..iterations {
            func();
        }
        let total_duration = start.elapsed();
        
        let avg_duration = total_duration / iterations as u32;
        println!("Benchmark {}: {} iterations, total: {:?}, avg: {:?}", 
                 name, iterations, total_duration, avg_duration);
        
        avg_duration
    }

    #[test]
    fn benchmark_basic_interpolation() {
        let perf_test = PerformanceTest::new();
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "test-app".to_string());
        variables.insert("version".to_string(), "1.0.0".to_string());
        
        let input = "name: ${name}, version: ${version}";
        
        let avg_duration = benchmark("basic_interpolation", 10000, || {
            let result = perf_test.interpolator.interpolate(input, &variables);
            assert!(result.is_ok());
        });
        
        // Basic interpolation should be very fast
        assert!(avg_duration < Duration::from_micros(100), 
                "Basic interpolation too slow: {:?}", avg_duration);
    }

    #[test]
    fn benchmark_nested_interpolation() {
        let perf_test = PerformanceTest::new();
        let mut variables = HashMap::new();
        variables.insert("base".to_string(), "app".to_string());
        variables.insert("env".to_string(), "prod".to_string());
        variables.insert("service".to_string(), "${base}-${env}".to_string());
        variables.insert("full_name".to_string(), "${service}-service".to_string());
        
        let input = "service: ${full_name}";
        
        let avg_duration = benchmark("nested_interpolation", 5000, || {
            let result = perf_test.interpolator.interpolate(input, &variables);
            assert!(result.is_ok());
        });
        
        // Nested interpolation should still be reasonably fast
        assert!(avg_duration < Duration::from_millis(1), 
                "Nested interpolation too slow: {:?}", avg_duration);
    }

    #[test]
    fn benchmark_yaml_parsing() {
        let perf_test = PerformanceTest::new();
        let yaml_content = r#"
variables:
  name: "benchmark-app"
  version: "1.0.0"
  env: "production"

config:
  app_name: ${name}
  app_version: ${version}
  environment: ${env}
  full_name: "${name}-${version}"

scripts:
  build: "echo Building ${name} ${version}"
  test: "echo Testing ${name}"
  deploy: "echo Deploying ${name} to ${env}"
"#;
        
        let avg_duration = benchmark("yaml_parsing", 1000, || {
            let result = perf_test.interpolator.parse_yaml_with_interpolation::<serde_yaml::Value>(yaml_content);
            assert!(result.is_ok());
        });
        
        // YAML parsing with interpolation should be fast
        assert!(avg_duration < Duration::from_millis(5), 
                "YAML parsing too slow: {:?}", avg_duration);
    }
}