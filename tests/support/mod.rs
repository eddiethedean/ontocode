use ontocore_catalog::{IndexBuilder, OntologyCatalog};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn fixture_workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

#[allow(dead_code)]
pub fn fixture_catalog() -> OntologyCatalog {
    IndexBuilder::new().workspace(fixture_workspace()).build().expect("index fixtures")
}

#[allow(dead_code)]
pub fn ontocore_binary() -> PathBuf {
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_ontocore") {
        let candidate = PathBuf::from(path);
        if candidate.exists() {
            return candidate;
        }
    }

    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| Path::new(env!("CARGO_MANIFEST_DIR")).join("target"));

    for subdir in ["debug", "release"] {
        let candidate = target_dir.join(subdir).join("ontocore");
        if candidate.exists() {
            return candidate;
        }
    }

    panic!(
        "ontocore binary not found under {} (run `cargo build -p ontocore-cli` first, or add ontocore-cli as a dev-dependency)",
        target_dir.display()
    );
}

/// Spawn the prebuilt `ontocore` CLI (avoids `cargo run` re-linking on every test invocation).
pub fn ontocore_cmd() -> Command {
    Command::new(ontocore_binary())
}
