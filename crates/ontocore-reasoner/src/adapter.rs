use crate::error::{ReasonerError, Result};
use crate::input::ReasonerInput;
use crate::result::{
    ClassificationResult, ConsistencyResult, ExplanationRequest, ExplanationResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasonerId {
    El,
    Rl,
    Rdfs,
    Dl,
    Auto,
}

impl ReasonerId {
    pub fn parse(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "el" => Ok(Self::El),
            "rl" => Ok(Self::Rl),
            "rdfs" => Ok(Self::Rdfs),
            "dl" => Ok(Self::Dl),
            "auto" => Ok(Self::Auto),
            _ => Err(ReasonerError::UnsupportedProfile(s.to_string())),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::El => "el",
            Self::Rl => "rl",
            Self::Rdfs => "rdfs",
            Self::Dl => "dl",
            Self::Auto => "auto",
        }
    }

    pub fn is_available(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasonerProfile {
    OwlEl,
    OwlRl,
    Rdfs,
    OwlDl,
    Auto,
}

impl From<ReasonerId> for ReasonerProfile {
    fn from(id: ReasonerId) -> Self {
        match id {
            ReasonerId::El => Self::OwlEl,
            ReasonerId::Rl => Self::OwlRl,
            ReasonerId::Rdfs => Self::Rdfs,
            ReasonerId::Dl => Self::OwlDl,
            ReasonerId::Auto => Self::Auto,
        }
    }
}

pub trait ReasonerAdapter: Send + Sync {
    fn id(&self) -> ReasonerId;
    fn profile(&self) -> ReasonerProfile;
    fn classify(&self, input: &ReasonerInput) -> Result<ClassificationResult>;
    fn check_consistency(&self, input: &ReasonerInput) -> Result<ConsistencyResult> {
        let result = self.classify(input)?;
        Ok(ConsistencyResult {
            consistent: result.unsatisfiable.is_empty(),
            unsatisfiable: result.unsatisfiable.clone(),
        })
    }
    fn explain(
        &self,
        input: &ReasonerInput,
        request: &ExplanationRequest,
    ) -> Result<ExplanationResult>;
}
