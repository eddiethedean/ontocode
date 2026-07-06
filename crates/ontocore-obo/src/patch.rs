use serde::{Deserialize, Serialize};

/// OBO patch operation (ADR-0019).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum OboPatchOp {
    SetName { term_id: String, value: String },
    AddSynonym { term_id: String, value: String, scope: String },
    RemoveSynonym { term_id: String, value: String },
    AddDef { term_id: String, value: String },
    RemoveDef { term_id: String },
    AddXref { term_id: String, xref: String },
    RemoveXref { term_id: String, xref: String },
    SetNamespace { term_id: String, namespace: String },
    SetDeprecated { term_id: String, value: bool },
    AddIsA { term_id: String, parent_id: String },
    RemoveIsA { term_id: String, parent_id: String },
}
