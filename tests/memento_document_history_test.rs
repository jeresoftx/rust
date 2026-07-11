use design_patterns_rust::patterns::gof::behavioral::memento::document_history::{
    DocumentEditor, DocumentHistory,
};

#[test]
fn memento_restores_previous_document_versions_from_history() {
    let mut editor = DocumentEditor::new("Incident report");
    let mut history = DocumentHistory::new();

    history.checkpoint(&editor);
    editor.replace_body("First draft");
    history.checkpoint(&editor);
    editor.replace_body("Final draft");

    assert_eq!(editor.body(), "Final draft");

    assert!(history.undo(&mut editor));
    assert_eq!(editor.body(), "First draft");

    assert!(history.undo(&mut editor));
    assert_eq!(editor.body(), "");

    assert!(!history.undo(&mut editor));
}

#[test]
fn memento_snapshot_preserves_title_body_and_tags() {
    let mut editor = DocumentEditor::new("Runbook");
    editor.replace_body("Restart service");
    editor.add_tag("ops");

    let snapshot = editor.save();
    editor.rename("Draft");
    editor.add_tag("temporary");
    editor.replace_body("Work in progress");

    editor.restore(snapshot);

    assert_eq!(editor.title(), "Runbook");
    assert_eq!(editor.body(), "Restart service");
    assert_eq!(editor.tags(), vec!["ops".to_string()]);
}
