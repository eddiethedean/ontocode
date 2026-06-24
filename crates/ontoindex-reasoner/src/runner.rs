use crate::adapter::{ReasonerAdapter, ReasonerId};
use crate::el::ElAdapter;
use crate::error::{ReasonerError, Result};
use crate::input::ReasonerInput;
use crate::rdfs::RdfsAdapter;
use crate::result::{ClassificationResult, ExplanationRequest, ExplanationResult};
use crate::rl::RlAdapter;
use crate::stub::{AutoAdapter, DlAdapter};
use ontologos_profile::scanner::scan_constructs;

pub fn adapter_for(id: ReasonerId) -> Result<Box<dyn ReasonerAdapter>> {
    if !id.is_available() {
        return Err(ReasonerError::RequiresOntoLogos1 { profile: id.as_str().to_string() });
    }
    let adapter: Box<dyn ReasonerAdapter> = match id {
        ReasonerId::El => Box::new(ElAdapter),
        ReasonerId::Rl => Box::new(RlAdapter),
        ReasonerId::Rdfs => Box::new(RdfsAdapter),
        ReasonerId::Dl => Box::new(DlAdapter),
        ReasonerId::Auto => Box::new(AutoAdapter),
    };
    Ok(adapter)
}

pub fn classify(
    profile: ReasonerId,
    input: &ReasonerInput,
    auto_detect: bool,
) -> Result<ClassificationResult> {
    let mut warnings = profile_warnings(&input.ontology, profile, auto_detect)?;
    if auto_detect {
        if let Some(suggested) = suggest_profile(&input.ontology) {
            if suggested != profile.as_str() {
                warnings.push(crate::result::ReasonerWarning {
                    code: "profile_suggestion".to_string(),
                    message: format!(
                        "ontology may be better suited to profile '{suggested}' (selected: {})",
                        profile.as_str()
                    ),
                    suggested_profile: Some(suggested),
                });
            }
        }
    }
    let adapter = adapter_for(profile)?;
    let mut result = adapter.classify(input)?;
    result.warnings.extend(warnings);
    Ok(result)
}

pub fn explain(
    profile: ReasonerId,
    input: &ReasonerInput,
    request: &ExplanationRequest,
) -> Result<ExplanationResult> {
    let adapter = adapter_for(profile)?;
    adapter.explain(input, request)
}

fn profile_warnings(
    ontology: &ontologos_core::Ontology,
    profile: ReasonerId,
    auto_detect: bool,
) -> Result<Vec<crate::result::ReasonerWarning>> {
    if !auto_detect {
        return Ok(Vec::new());
    }
    let report = ontologos_profile::detect_profile(ontology)
        .map_err(|e| ReasonerError::Classify(e.to_string()))?;
    let mut warnings = Vec::new();
    for diag in report.diagnostics {
        warnings.push(crate::result::ReasonerWarning {
            code: "profile_construct".to_string(),
            message: diag.message,
            suggested_profile: report.detected.map(|p| format!("{p:?}").to_ascii_lowercase()),
        });
    }
    if profile == ReasonerId::El {
        let constructs = scan_constructs(ontology);
        for diag in ontologos_profile::el_diagnostics(&constructs) {
            warnings.push(crate::result::ReasonerWarning {
                code: "el_construct".to_string(),
                message: diag.message,
                suggested_profile: Some("el".to_string()),
            });
        }
    }
    Ok(warnings)
}

fn suggest_profile(ontology: &ontologos_core::Ontology) -> Option<String> {
    let report = ontologos_profile::detect_profile(ontology).ok()?;
    report.detected.map(|p| match p {
        ontologos_profile::OwlProfile::El => "el".to_string(),
        ontologos_profile::OwlProfile::Rl => "rl".to_string(),
        ontologos_profile::OwlProfile::Ql => "el".to_string(),
        ontologos_profile::OwlProfile::Dl => "dl".to_string(),
    })
}
