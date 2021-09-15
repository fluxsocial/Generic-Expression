use hdk::prelude::*;

entry_defs![Schema::entry_def()];
#[hdk_entry(id = "schema_validation", visibility = "public")]
pub struct Schema {
    definition: String,
}

#[hdk_extern]
pub fn create_schema(schema: Schema) -> ExternResult<EntryHash> {

    // todo validate schema definition
    let _header_hash = create_entry(&schema)?;

    // todo create link

    hash_entry(schema)
}

pub fn validate_content(schema_entry_hash: EntryHash, _content: String) -> ExternResult<bool> {
    let element = get(schema_entry_hash, GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Could not find the schema")))?;

    let maybe_schema = element.entry().to_app_option()?;
    let _schema: Schema = maybe_schema.expect("Schema should exist once found");

    // todo validate content
    Ok(true)
}