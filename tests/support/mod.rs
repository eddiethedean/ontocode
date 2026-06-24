use ontoindex_catalog::{IndexBuilder, OntologyCatalog};
use std::path::{Path, PathBuf};

pub fn fixture_workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

#[allow(dead_code)]
pub fn fixture_catalog() -> OntologyCatalog {
    IndexBuilder::new().workspace(fixture_workspace()).build().expect("index fixtures")
}

#[allow(dead_code)]
pub fn ontoindex_binary() -> PathBuf {
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_ontoindex") {
        let candidate = PathBuf::from(path);
        if candidate.exists() {
            return candidate;
        }
    }

    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| Path::new(env!("CARGO_MANIFEST_DIR")).join("target"));

    for subdir in ["debug", "release"] {
        let candidate = target_dir.join(subdir).join("ontoindex");
        if candidate.exists() {
            return candidate;
        }
    }

    panic!(
        "ontoindex binary not found under {} (run `cargo build -p ontoindex-cli` first)",
        target_dir.display()
    );
}
