use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use design_patterns_rust::patterns::rust_idiomatic::raii::temporary_file_cleanup::TemporaryFile;

#[test]
fn raii_removes_temporary_file_when_value_drops() {
    let path = unique_path("drop");

    {
        let file = TemporaryFile::create(&path, "draft report").expect("file should be created");

        assert!(path.exists());
        assert_eq!(file.read_to_string().unwrap(), "draft report");
    }

    assert!(!path.exists());
}

#[test]
fn raii_cleans_temporary_file_on_early_return() {
    fn export_preview(path: &PathBuf) {
        let file = TemporaryFile::create(path, "preview").expect("file should be created");
        file.append_line("confirmed").unwrap();

        return;
    }

    let path = unique_path("early-return");

    export_preview(&path);

    assert!(!path.exists());
}

#[test]
fn raii_drop_tolerates_file_already_removed() {
    let path = unique_path("manual-remove");

    {
        let file = TemporaryFile::create(&path, "cache").expect("file should be created");

        assert!(file.path().exists());
        fs::remove_file(&path).unwrap();
    }

    assert!(!path.exists());
}

fn unique_path(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after epoch")
        .as_nanos();

    std::env::temp_dir().join(format!("design-patterns-rust-raii-{label}-{nanos}.txt"))
}
