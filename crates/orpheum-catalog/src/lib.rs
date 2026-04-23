#[derive(Debug, Clone, Copy)]
pub struct EmbeddedCatalogFile {
    pub path: &'static str,
    pub contents: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/embedded_catalog.rs"));

pub fn embedded_catalog_files() -> &'static [EmbeddedCatalogFile] {
    EMBEDDED_CATALOG_FILES
}
