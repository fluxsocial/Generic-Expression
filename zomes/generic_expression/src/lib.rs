use generic_expression_integrity::{EntryTypes, Expression, LinkTypes};
use hdk::prelude::*;
use serde_json::Value;

mod config;
mod constants;
mod params;
mod schema;
mod utils;

use config::*;
use constants::*;
use params::*;
use schema::validate_content;
use utils::*;

/// Run function when zome is initialized by agent.
/// This adds open cap grant for recv_private_expression function
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn create_expression(input: ExpressionInput) -> ExternResult<EntryHash> {
    let ExpressionInput {
        data,
        author,
        timestamp,
        proof,
    } = input;

    let data_json: Value = serde_json::from_str(&data).map_err(|e| err(&e.to_string()))?;
    validate_content(&EXPRESSION_DATA_SCHEMA, &data_json)?;

    let expression = Expression {
        data: data_json,
        author,
        timestamp,
        proof,
    };

    let entry_hash = hash_entry(&expression)?;
    let _header_hash = create_entry(&EntryTypes::Expression(expression.clone()))?;

    hc_time_index::index_entry(
        expression.author.clone(),
        expression.clone(),
        LinkTag::new(EXPRESSION_TAG_NAME),
        LinkTypes::Expression,
        LinkTypes::Index,
    )
    .map_err(|e| err(&e.to_string()))?;

    Ok(entry_hash)
}

#[hdk_extern]
pub fn get_expression_by_author(input: GetByAuthorInput) -> ExternResult<Vec<Expression>> {
    let links = hc_time_index::get_links_for_time_span(
        input.author,
        input.from,
        input.until,
        Some(LinkTag::new(EXPRESSION_TAG_NAME)),
        None,
        LinkTypes::Expression,
        LinkTypes::Index,
    )
    .map_err(|e| err(&e.to_string()))?;
    // debug!("Got links: {:#?}", links);
    links
        .into_iter()
        .map(|link| {
            let element = get(link.target, GetOptions::default())?
                .ok_or(err("Could not get entry after commit."))?;
            let expression = element
                .entry()
                .to_app_option::<Expression>()
                .map_err(|err| wasm_error!(WasmErrorInner::Host(err.to_string())))?
                .ok_or(wasm_error!(WasmErrorInner::Host(
                    "Expected record to contain app data".to_string()
                )))?;
            Ok(expression)
        })
        .collect()
}

#[hdk_extern]
pub fn get_expression_by_address(input: EntryHash) -> ExternResult<Option<Expression>> {
    let optional_element = get(input, GetOptions::default())?;
    if let Some(element) = optional_element {
        let expression: Expression = element
            .entry()
            .to_app_option::<Expression>()
            .map_err(|err| wasm_error!(WasmErrorInner::Host(err.to_string())))?
            .ok_or(wasm_error!(WasmErrorInner::Host(
                "Expected record to contain app data".to_string()
            )))?;

        return Ok(Some(expression));
    }

    Ok(None)
}
