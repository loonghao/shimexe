use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shimexe_core::{ShimConfig, ShimRunner};
use std::collections::HashMap;
use std::io::Write;
use tempfile::NamedTempFile;

fn create_test_config() -> ShimConfig {
    ShimConfig {
        shim: shimexe_core::config::ShimCore {
            name: "test-shim".to_string(),
            path: "echo".to_string(),
            args: vec!["hello".to_string(), "world".to_string()],
            cwd: None,
            download_url: None,
        },
        args: Default::default(),
        env: {
            let mut env = HashMap::new();
            env.insert("TEST_VAR".to_string(), "test_value".to_string());
            env.insert("PATH_VAR".to_string(), "/usr/bin:/bin".to_string());
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
name = "test-shim"
path = "echo"
args = ["hello", "world"]

[env]
TEST_VAR = "test_value"
PATH_VAR = "/usr/bin:/bin"
ANOTHER_VAR = "another_value"
CONFIG_PATH = "/etc/config"
LOG_LEVEL = "debug"

[metadata]
description = "Test shim for benchmarking"
version = "1.0.0"
author = "Test Author"
        "#
    )
    .unwrap();
    temp_file
}

fn bench_config_loading(c: &mut Criterion) {
    let temp_file = create_test_config_file();
    
    c.bench_function("config_from_file", |b| {
        b.iter(|| {
            let config = ShimConfig::from_file(black_box(temp_file.path())).unwrap();
            black_box(config);
        })
    });
}

fn bench_config_saving(c: &mut Criterion) {
    let config = create_test_config();
    
    c.bench_function("config_to_file", |b| {
        b.iter(|| {
            let temp_file = NamedTempFile::new().unwrap();
            config.to_file(black_box(temp_file.path())).unwrap();
            black_box(temp_file);
        })
    });
}

fn bench_runner_creation(c: &mut Criterion) {
    let config = create_test_config();
    
    c.bench_function("runner_from_config", |b| {
        b.iter(|| {
            let runner = ShimRunner::from_config(black_box(config.clone())).unwrap();
            black_box(runner);
        })
    });
    
    let temp_file = create_test_config_file();
    c.bench_function("runner_from_file", |b| {
        b.iter(|| {
            let runner = ShimRunner::from_file(black_box(temp_file.path())).unwrap();
            black_box(runner);
        })
    });
}

fn bench_validation(c: &mut Criterion) {
    let config = create_test_config();
    let runner = ShimRunner::from_config(config).unwrap();
    
    c.bench_function("validate_executable", |b| {
        b.iter(|| {
            let result = runner.validate();
            black_box(result);
        })
    });
}

fn bench_config_expansion(c: &mut Criterion) {
    let mut config = create_test_config();
    // Add some environment variables that need expansion
    config.shim.path = "${HOME}/bin/echo".to_string();
    config.shim.args = vec!["${USER}".to_string(), "${PWD}".to_string()];
    
    c.bench_function("expand_env_vars", |b| {
        b.iter(|| {
            let mut config_copy = config.clone();
            let result = config_copy.expand_env_vars();
            black_box(result);
        })
    });
}

fn bench_concurrent_config_loading(c: &mut Criterion) {
    use shimexe_core::ShimConfig;
    use std::path::PathBuf;
    
    // Create multiple test files
    let temp_files: Vec<_> = (0..10).map(|_| create_test_config_file()).collect();
    let paths: Vec<PathBuf> = temp_files.iter().map(|f| f.path().to_path_buf()).collect();
    
    c.bench_function("concurrent_config_loading", |b| {
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let results = ShimConfig::from_files_concurrent(black_box(paths.clone())).await;
                black_box(results);
            });
        })
    });
}

fn bench_cache_performance(c: &mut Criterion) {
    let config = create_test_config();
    let runner = ShimRunner::from_config(config).unwrap();
    
    // First validation to populate cache
    let _ = runner.validate();
    
    c.bench_function("cached_validation", |b| {
        b.iter(|| {
            let result = runner.validate();
            black_box(result);
        })
    });
}

criterion_group!(
    benches,
    bench_config_loading,
    bench_config_saving,
    bench_runner_creation,
    bench_validation,
    bench_config_expansion,
    bench_concurrent_config_loading,
    bench_cache_performance
);
criterion_main!(benches);
