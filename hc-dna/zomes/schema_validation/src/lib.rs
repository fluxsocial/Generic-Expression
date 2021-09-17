use hdk::prelude::*;
use lazy_static::lazy_static;
use jsonschema_valid::{schemas, Config};
use serde_json::Value;

entry_defs![Expression::entry_def()];

// schema json validate entry
// - field not too big, max length
// - 


#[derive(SerializedBytes, Serialize, Deserialize, Clone, Debug)]
pub struct CreateExpressionInput {
    pub data: String,
    // pub author: String,
    // pub timestamp: DateTime<Utc>,
    // pub proof: ExpressionProof,
}

#[hdk_entry(id = "expression", visibility = "public")]
pub struct Expression {
    pub data: Value,
}

#[derive(SerializedBytes, Serialize, Deserialize, Clone, Debug)]
pub struct ExpressionProof {
    pub signature: String,
    pub key: String,
}

#[hdk_extern]
pub fn create_expression(input: CreateExpressionInput) -> ExternResult<String> {
    // let CreateExpressionInput { data } = input;

    let schema: Value = serde_json::from_str(&EXPRESSION_SCHEMA)
        .map_err(|e| WasmError::Host(e.to_string()))?;
    let cfg = Config::from_schema(&schema, Some(schemas::Draft::Draft7))
        .map_err(|e| WasmError::Host(e.to_string()))?;

    assert!(cfg.validate_schema().is_ok());
    
    let data_value: Value = serde_json::from_str(&input.data)
        .map_err(|e| WasmError::Host(e.to_string()))?;
    assert!(cfg.validate(&data_value).is_ok());

    let entry = Expression {
        data: data_value,
    };

    let _entry_hash = hash_entry(&entry)?;
    let _header_hash = create_entry(entry)?;
    // no transactional, verify first, write last.
    
    Ok(EXPRESSION_SCHEMA.to_string())
}

#[derive(SerializedBytes, Serialize, Deserialize, Debug)]
pub struct Properties {
    pub expression_data_schema: String,
}

lazy_static! {
    pub static ref EXPRESSION_SCHEMA: String = {
        let host_dna_config = zome_info()
            .expect("Could not get zome configuration.")
            .properties;
        let properties = Properties::try_from(host_dna_config)
            .expect("Could not convert zome dna properties to Properties.");
        properties.expression_data_schema
    };
}
