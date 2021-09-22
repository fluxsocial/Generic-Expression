use hdk::prelude::*;
use serde_json::Value;
use jsonschema_valid::{schemas, Config};
use crate::utils::err;

pub fn validate_content(schema: &Value, content: &Value) -> ExternResult<()> {
    let cfg = Config::from_schema(schema, Some(schemas::Draft::Draft7))
        .map_err(|e| err(&e.to_string()))?;

    cfg.validate_schema()
        .map_err(|_| err("JSON Schema is invalid."))?;
    cfg.validate(content)
        .map_err(|_| err("Content is invalid by JSON Schema."))?;

    Ok(())
}
