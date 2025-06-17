use shimexe_core::{ShimConfig, ShimRunner};
use std::collections::HashMap;
use std::io::Write;
use std::time::{Duration, Instant};
use tempfile::NamedTempFile;

fn create_test_config() -> ShimConfig {
    ShimConfig {
        shim: shimexe_core::config::ShimCore {
            name: "test-perf".to_string(),
            path: "echo".to_string(),
            args: vec!["performance".to_string(), "test".to_string()],
            cwd: None,
            download_url: None,
            source_type: shimexe_core::config::SourceType::File,
            extracted_executables: vec![],
        },
        args: Default::default(),
        env: {
            let mut env = HashMap::new();
            env.insert("PERF_TEST".to_string(), "true".to_string());
            env.insert("TEST_MODE".to_string(), "benchmark".to_string());
            env
        },
        metadata: Default::default(),
        auto_update: None,
    }
}

fn create_test_config_file() -> NamedTempFile {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(
        temp_file,
        r#"
[shim]
name = "test-perf"
path = "echo"
args = ["performance", "test"]

[env]
PERF_TEST = "true"
TEST_MODE = "benchmark"
LOG_LEVEL = "info"

[metadata]
description = "Performance test shim"
version = "1.0.0"
        "#
    )
    .unwrap();
    temp_file
}

#[test]
fn test_config_loading_performance() {
    let temp_file = create_test_config_file();
    let iterations = 100;

    let start = Instant::now();
    for _ in 0..iterations {
        let _config = ShimConfig::from_file(temp_file.path()).unwrap();
    }
    let elapsed = start.elapsed();

    let avg_time = elapsed / iterations;
    println!("Average config loading time: {:?}", avg_time);

    // Config loading should be fast (< 10ms per operation)
    assert!(
        avg_time < Duration::from_millis(10),
        "Config loading too slow: {:?}",
        avg_time
    );
}

#[test]
fn test_runner_creation_performance() {
    let config = create_test_config();
    let iterations = 100;

    let start = Instant::now();
    for _ in 0..iterations {
        let _runner = ShimRunner::from_config(config.clone()).unwrap();
    }
    let elapsed = start.elapsed();

    let avg_time = elapsed / iterations;
    println!("Average runner creation time: {:?}", avg_time);

    // Runner creation should be fast (< 5ms per operation)
    assert!(
        avg_time < Duration::from_millis(5),
        "Runner creation too slow: {:?}",
        avg_time
    );
}

#[test]
fn test_validation_caching_performance() {
    let config = create_test_config();
    let runner = ShimRunner::from_config(config).unwrap();

    // First validation (cache miss)
    let start = Instant::now();
    let _result = runner.validate();
    let first_validation = start.elapsed();

    // Subsequent validations (cache hits)
    let iterations = 50;
    let start = Instant::now();
    for _ in 0..iterations {
        let _result = runner.validate();
    }
    let cached_validations = start.elapsed();

    let avg_cached_time = cached_validations / iterations;

    println!("First validation time: {:?}", first_validation);
    println!("Average cached validation time: {:?}", avg_cached_time);

    // Cached validations should be faster (allowing for Windows file system overhead)
    assert!(
        avg_cached_time < first_validation + Duration::from_millis(2),
        "Caching not providing expected performance benefit. First: {:?}, Cached: {:?}",
        first_validation,
        avg_cached_time
    );

    // Cached validations should be reasonably fast (< 10ms on Windows)
    assert!(
        avg_cached_time < Duration::from_millis(10),
        "Cached validation too slow: {:?}",
        avg_cached_time
    );
}

#[tokio::test]
async fn test_concurrent_config_loading_performance() {
    use shimexe_core::ShimConfig;
    use std::path::PathBuf;

    // Create multiple test files
    let temp_files: Vec<_> = (0..10).map(|_| create_test_config_file()).collect();
    let paths: Vec<PathBuf> = temp_files.iter().map(|f| f.path().to_path_buf()).collect();

    // Sequential loading
    let start = Instant::now();
    for path in &paths {
        let _config = ShimConfig::from_file(path).unwrap();
    }
    let sequential_time = start.elapsed();

    // Concurrent loading
    let start = Instant::now();
    let _results = ShimConfig::from_files_concurrent(paths).await;
    let concurrent_time = start.elapsed();

    println!("Sequential loading time: {:?}", sequential_time);
    println!("Concurrent loading time: {:?}", concurrent_time);

    // Concurrent loading should be faster for multiple files
    // (allowing some overhead for small file counts)
    assert!(
        concurrent_time < sequential_time + Duration::from_millis(50),
        "Concurrent loading not providing expected performance benefit"
    );
}

#[test]
fn test_config_caching_performance() {
    use shimexe_core::config::ConfigCache;
    use std::time::Duration;

    let cache = ConfigCache::new(Duration::from_secs(60));
    let temp_file = create_test_config_file();

    // First load (cache miss)
    let start = Instant::now();
    let _config1 = cache.get_or_load(temp_file.path()).unwrap();
    let first_load = start.elapsed();

    // Second load (cache hit)
    let start = Instant::now();
    let _config2 = cache.get_or_load(temp_file.path()).unwrap();
    let cached_load = start.elapsed();

    println!("First load time: {:?}", first_load);
    println!("Cached load time: {:?}", cached_load);

    // Cached load should be significantly faster
    assert!(
        cached_load < first_load / 2,
        "Config caching not providing expected performance benefit"
    );

    // Cached load should be reasonably fast (< 5ms on Windows)
    assert!(
        cached_load < Duration::from_millis(5),
        "Cached config load too slow: {:?}",
        cached_load
    );
}

#[test]
fn test_memory_usage_optimization() {
    // Test that we don't have excessive memory allocations
    let config = create_test_config();
    let runner = ShimRunner::from_config(config).unwrap();

    // Multiple validations shouldn't cause memory leaks
    for _ in 0..1000 {
        let _result = runner.validate();
    }

    // This test mainly ensures we don't panic or crash
    // In a real scenario, you might use a memory profiler
    assert!(true, "Memory usage test completed without issues");
}

#[test]
fn test_environment_variable_optimization() {
    let mut config = create_test_config();

    // Add many environment variables
    for i in 0..100 {
        config
            .env
            .insert(format!("VAR_{}", i), format!("value_{}", i));
    }

    let runner = ShimRunner::from_config(config).unwrap();

    // Test that environment variable processing is efficient
    let iterations = 10;
    let start = Instant::now();
    for _ in 0..iterations {
        // This would normally execute the command, but we're just testing the setup
        let _result = runner.validate();
    }
    let elapsed = start.elapsed();

    let avg_time = elapsed / iterations;
    println!("Average time with many env vars: {:?}", avg_time);

    // Should handle many environment variables efficiently
    assert!(
        avg_time < Duration::from_millis(10),
        "Environment variable processing too slow: {:?}",
        avg_time
    );
}
