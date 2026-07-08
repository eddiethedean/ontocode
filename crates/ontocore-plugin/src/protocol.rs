use ontocore_core::{Diagnostic, DiagnosticCode, DiagnosticSeverity, SourceLocation};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// JSON wire format returned by subprocess plugins on stdout.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginOutput {
    #[serde(default)]
    pub diagnostics: Vec<PluginDiagnosticWire>,
    #[serde(default)]
    pub output_paths: Vec<String>,
    #[serde(default)]
    pub logs: Option<String>,
    #[serde(default)]
    pub exit_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDiagnosticWire {
    pub code: String,
    pub severity: String,
    pub message: String,
    pub file: String,
    #[serde(default)]
    pub line: Option<u64>,
    #[serde(default)]
    pub column: Option<u64>,
    #[serde(default)]
    pub entity_iri: Option<String>,
}

impl PluginDiagnosticWire {
    pub fn into_diagnostic(self, plugin_id: &str, workspace: &Path) -> Diagnostic {
        let file = if Path::new(&self.file).is_absolute() {
            PathBuf::from(&self.file)
        } else {
            workspace.join(&self.file)
        };
        Diagnostic {
            code: DiagnosticCode::PluginViolation,
            severity: match self.severity.as_str() {
                "error" => DiagnosticSeverity::Error,
                "warning" => DiagnosticSeverity::Warning,
                _ => DiagnosticSeverity::Info,
            },
            message: self.message,
            file,
            range: SourceLocation {
                line: self.line,
                column: self.column,
                start_byte: None,
                end_byte: None,
            },
            entity_iri: self.entity_iri,
            quick_fix: None,
            plugin_id: Some(plugin_id.to_string()),
            plugin_code: Some(self.code),
        }
    }
}

pub fn plugin_diagnostic(
    plugin_id: &str,
    code: &str,
    severity: DiagnosticSeverity,
    message: impl Into<String>,
    file: PathBuf,
    entity_iri: Option<String>,
) -> Diagnostic {
    Diagnostic {
        code: DiagnosticCode::PluginViolation,
        severity,
        message: message.into(),
        file,
        range: SourceLocation::default(),
        entity_iri,
        quick_fix: None,
        plugin_id: Some(plugin_id.to_string()),
        plugin_code: Some(code.to_string()),
    }
}
