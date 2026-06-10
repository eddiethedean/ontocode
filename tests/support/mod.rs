use ontoindex_catalog::{IndexBuilder, OntologyCatalog};
use std::path::PathBuf;

pub fn fixture_workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

pub fn fixture_catalog() -> OntologyCatalog {
    IndexBuilder::new().workspace(fixture_workspace()).build().expect("index fixtures")
}
