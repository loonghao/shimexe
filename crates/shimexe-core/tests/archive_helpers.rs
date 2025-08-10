// Additional coverage for ArchiveExtractor helpers

use shimexe_core::archive::ArchiveExtractor;
use std::fs;
use std::path::Path;

#[test]
fn test_find_executables_in_dir_empty_and_basic() {
    let tmp = tempfile::tempdir().unwrap();

    // Empty dir returns empty
    let list = ArchiveExtractor::find_executables_in_dir(tmp.path()).unwrap();
    assert!(list.is_empty());

    // Create some files
    let exe = tmp.path().join("tool.exe");
    let txt = tmp.path().join("readme.txt");
    fs::write(&exe, b"ok").unwrap();
    fs::write(&txt, b"text").unwrap();

    let list2 = ArchiveExtractor::find_executables_in_dir(tmp.path()).unwrap();
    // Should include .exe but not .txt
    assert!(list2.iter().any(|p| p.ends_with(Path::new("tool.exe"))));
    assert!(!list2.iter().any(|p| p.ends_with(Path::new("readme.txt"))));
}

#[test]
fn test_generate_shim_name_collision() {
    let existing = vec!["app".to_string(), "app-1".to_string(), "app-2".to_string()];
    let name = ArchiveExtractor::generate_shim_name(Path::new("app.exe"), &existing);
    assert_eq!(name, "app-3");
}

