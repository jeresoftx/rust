//! Catalogo didactico de patrones en Rust.

pub mod patterns;

/// Familias principales del catalogo en orden de aprendizaje.
pub fn catalog_families() -> [&'static str; 3] {
    ["gof", "rust_idiomatic", "architecture"]
}
