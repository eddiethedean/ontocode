//! Workspace diagnostic rule configuration (`.ontocore/diagnostics.toml`).

use ontocore_core::{DiagnosticCode, DiagnosticSeverity};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct DiagnosticConfig {
    #[serde(default)]
    pub rules: HashMap<String, RuleConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuleConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub severity: Option<String>,
}

fn default_enabled() -> bool {
    true
}

impl DiagnosticConfig {
    pub fn load(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
        toml::from_str(&text).map_err(|e| format!("invalid diagnostics config: {e}"))
    }

    pub fn is_rule_enabled(&self, code: DiagnosticCode) -> bool {
        let key = code.as_str();
        self.rules.get(key).map(|r| r.enabled).unwrap_or(true)
    }

    pub fn severity_override(&self, code: DiagnosticCode) -> Option<DiagnosticSeverity> {
        let key = code.as_str();
        let sev = self.rules.get(key)?.severity.as_deref()?;
        match sev.to_ascii_lowercase().as_str() {
            "error" => Some(DiagnosticSeverity::Error),
            "warning" => Some(DiagnosticSeverity::Warning),
            "info" | "hint" => Some(DiagnosticSeverity::Info),
            _ => None,
        }
    }
}

pub fn find_config(workspace: &Path) -> Option<DiagnosticConfig> {
    let path = workspace.join(".ontocore").join("diagnostics.toml");
    DiagnosticConfig::load(&path).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_rule_toggles() {
        let cfg: DiagnosticConfig = toml::from_str(
            r#"
[rules.missing_label]
enabled = false
[rules.broken_import]
severity = "error"
"#,
        )
        .unwrap();
        assert!(!cfg.is_rule_enabled(DiagnosticCode::MissingLabel));
        assert_eq!(
            cfg.severity_override(DiagnosticCode::BrokenImport),
            Some(DiagnosticSeverity::Error)
        );
    }
}
