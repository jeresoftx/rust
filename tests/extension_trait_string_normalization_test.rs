use design_patterns_rust::patterns::rust_idiomatic::extension_trait::string_normalization::StringNormalizationExt;

#[test]
fn extension_trait_normalizes_whitespace_in_user_input() {
    let input = "  Orden    con   espacios   ";

    assert_eq!(input.normalized_spaces(), "Orden con espacios");
}

#[test]
fn extension_trait_normalizes_email_like_business_input() {
    let email = String::from("  ADMIN@Example.COM  ");

    assert_eq!(email.normalized_email(), "admin@example.com");
}

#[test]
fn extension_trait_builds_stable_slug_keys() {
    let title = "  Reporte Mensual: Ventas & Cobranza  ";

    assert_eq!(title.slug_key(), "reporte-mensual-ventas-cobranza");
}
