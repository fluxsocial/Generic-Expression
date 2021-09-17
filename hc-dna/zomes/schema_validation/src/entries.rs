use hdk::prelude::*;
use hc_time_index::IndexableEntry;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(SerializedBytes, Serialize, Deserialize, Clone, Debug)]
pub struct ExpressionProof {
    pub signature: String,
    pub key: String,
}

#[hdk_entry(id = "expression", visibility = "public")]
#[derive(Clone)]
pub struct Expression {
    pub data: Value,
    pub author: String,
    pub timestamp: DateTime<Utc>,
    pub proof: ExpressionProof,
}

impl IndexableEntry for Expression {
    fn entry_time(&self) -> DateTime<Utc> {
        self.timestamp
    }

    fn hash(&self) -> ExternResult<EntryHash> {
        hash_entry(self)
    }
}

