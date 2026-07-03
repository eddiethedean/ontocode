use crate::adapter::{ReasonerAdapter, ReasonerId, ReasonerProfile};
use crate::error::{ReasonerError, Result};
use crate::input::ReasonerInput;
use crate::result::{ClassificationResult, ExplanationRequest, ExplanationResult};

pub struct DlAdapter;

impl ReasonerAdapter for DlAdapter {
    fn id(&self) -> ReasonerId {
        ReasonerId::Dl
    }

    fn profile(&self) -> ReasonerProfile {
        ReasonerProfile::OwlDl
    }

    fn classify(&self, _input: &ReasonerInput) -> Result<ClassificationResult> {
        Err(ReasonerError::RequiresOntoLogos1 { profile: "dl".to_string() })
    }

    fn explain(
        &self,
        _input: &ReasonerInput,
        _request: &ExplanationRequest,
    ) -> Result<ExplanationResult> {
        Err(ReasonerError::RequiresOntoLogos1 { profile: "dl".to_string() })
    }
}

pub struct AutoAdapter;

impl ReasonerAdapter for AutoAdapter {
    fn id(&self) -> ReasonerId {
        ReasonerId::Auto
    }

    fn profile(&self) -> ReasonerProfile {
        ReasonerProfile::Auto
    }

    fn classify(&self, _input: &ReasonerInput) -> Result<ClassificationResult> {
        Err(ReasonerError::RequiresOntoLogos1 { profile: "auto".to_string() })
    }

    fn explain(
        &self,
        _input: &ReasonerInput,
        _request: &ExplanationRequest,
    ) -> Result<ExplanationResult> {
        Err(ReasonerError::RequiresOntoLogos1 { profile: "auto".to_string() })
    }
}
