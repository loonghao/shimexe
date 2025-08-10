// 2025 tech: Rust stable. Test ConfigCache cache hit/miss and invalidation.

use shimexe_core::config::{ConfigCache, ShimConfig, ShimCore, SourceType};
use std::time::Duration;
use tempfile::NamedTempFile;

fn write_config(tmp: &mut NamedTempFile, name: &str) {
    let cfg = ShimConfig {
        shim: ShimCore {
            name: name.to_string(),
            path: "echo".to_string(),
            args: vec![],
            cwd: None,
            download_url: None,
            source_type: SourceType::File,
            extracted_executables: vec![],
        },
        args: Default::default(),
        env: Default::default(),
        metadata: Default::default(),
        auto_update: None,
    };
    cfg.to_file(tmp.path()).unwrap();
}

#[test]
fn test_cache_hit_and_invalidate() {
    let mut tmp = NamedTempFile::new().unwrap();
    write_config(&mut tmp, "one");

    let cache = ConfigCache::new(Duration::from_secs(60));

    let first = cache.get_or_load(tmp.path()).unwrap();
    assert_eq!(first.shim.name, "one");

    // Modify file and ensure cache invalidates after manual invalidate
    write_config(&mut tmp, "two");

    // Without invalidate, still might return cached if timestamp within TTL and unchanged
    // Force invalidate to ensure reload
    cache.invalidate(tmp.path());
    let second = cache.get_or_load(tmp.path()).unwrap();
    assert_eq!(second.shim.name, "two");

    // Stats present and valid count <= total
    let (total, valid) = cache.stats();
    assert!(total >= 1);
    assert!(valid <= total);
}
