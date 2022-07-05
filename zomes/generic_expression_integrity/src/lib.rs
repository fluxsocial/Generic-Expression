use chrono::{DateTime, Utc};
use hc_time_index::IndexableEntry;
use hdk::prelude::*;
use serde_json::Value;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq, Hash, SerializedBytes)]
pub struct ExpressionProof {
    pub signature: String,
    pub key: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq, SerializedBytes)]
pub struct Expression {
    pub data: Value,
    pub author: String,
    pub timestamp: DateTime<Utc>,
    pub proof: ExpressionProof,
}

app_entry!(Expression);

impl IndexableEntry for Expression {
    fn entry_time(&self) -> DateTime<Utc> {
        self.timestamp
    }

    fn hash(&self) -> ExternResult<EntryHash> {
        hash_entry(self)
    }
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def(visibility = "public")]
    Expression(Expression),
}

#[hdk_link_types]
pub enum LinkTypes {
    Index,
    Expression,
}
