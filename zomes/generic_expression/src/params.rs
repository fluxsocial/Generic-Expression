use chrono::{DateTime, Utc};
use generic_expression_integrity::ExpressionProof;
use hdk::prelude::*;

#[derive(SerializedBytes, Serialize, Deserialize, Clone, Debug)]
pub struct ExpressionInput {
    pub data: String,
    pub author: String,
    pub timestamp: DateTime<Utc>,
    pub proof: ExpressionProof,
}

#[derive(SerializedBytes, Serialize, Deserialize, Clone, Debug)]
pub struct GetByAuthorInput {
    pub author: String,
    pub from: DateTime<Utc>,
    pub until: DateTime<Utc>,
}

#[derive(SerializedBytes, Serialize, Deserialize, Clone, Debug)]
pub struct PrivateExpressionInput {
    pub to: AgentPubKey,
    pub expression: ExpressionInput,
}

#[derive(SerializedBytes, Serialize, Deserialize, Clone, Debug)]
pub struct InboxInput {
    pub from: Option<String>,
    pub page_size: usize,
    pub page_number: usize,
}
