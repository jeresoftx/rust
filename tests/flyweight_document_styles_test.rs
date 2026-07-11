use design_patterns_rust::patterns::gof::structural::flyweight::document_styles::{
    DocumentStyleRegistry, TextRun,
};

#[test]
fn flyweight_reuses_document_styles_across_text_runs() {
    let mut registry = DocumentStyleRegistry::new();
    let heading_style = registry.style("heading", "Inter", 24, true);

    let first = TextRun::new("Revenue", heading_style.clone());
    let second = TextRun::new("Costs", registry.style("heading", "ignored", 12, false));

    assert!(first.shares_style_with(&second));
    assert_eq!(first.render(), "[heading Inter 24 bold] Revenue");
    assert_eq!(second.render(), "[heading Inter 24 bold] Costs");
}

#[test]
fn flyweight_keeps_text_outside_shared_style() {
    let mut registry = DocumentStyleRegistry::new();
    let body_style = registry.style("body", "Serif", 12, false);

    let run = TextRun::new("Quarterly summary", body_style);

    assert_eq!(run.render(), "[body Serif 12 regular] Quarterly summary");
}
