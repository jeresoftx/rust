use design_patterns_rust::patterns::gof::structural::composite::file_system::{FileEntry, Folder};

#[test]
fn composite_calculates_total_size_for_nested_folders() {
    let folder = Folder::new("root")
        .with(FileEntry::new("readme.md", 120))
        .with(Folder::new("src").with(FileEntry::new("main.rs", 350)));

    assert_eq!(folder.total_size(), 470);
}

#[test]
fn composite_lists_file_paths_inside_subfolders() {
    let folder = Folder::new("root")
        .with(FileEntry::new("readme.md", 120))
        .with(
            Folder::new("src")
                .with(FileEntry::new("main.rs", 350))
                .with(Folder::new("tests").with(FileEntry::new("mod.rs", 80))),
        );

    assert_eq!(
        folder.file_paths(),
        vec![
            "root/readme.md".to_string(),
            "root/src/main.rs".to_string(),
            "root/src/tests/mod.rs".to_string(),
        ]
    );
}
