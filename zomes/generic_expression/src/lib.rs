use hdk::prelude::*;
use serde_json::Value;

mod entries;
mod params;
mod config;
mod schema;
mod constants;
mod utils;

use entries::*;
use params::*;
use config::*;
use constants::*;
use utils::*;
use schema::validate_content;

entry_defs![
    Expression::entry_def(),
    PrivateExpression::entry_def(),
    PrivateAcaiAgent::entry_def(),
    Path::entry_def()
];

/// Run function when zome is initialized by agent.
/// This adds open cap grant for recv_private_expression function
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut functions: GrantedFunctions = BTreeSet::new();
    functions.insert((zome_info()?.name, RECV_PRIVATE_EXPRESSION_FUNC_NAME.into()));
    
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // Empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn create_expression(input: ExpressionInput) -> ExternResult<EntryHash> {
    let ExpressionInput { data, author, timestamp, proof } = input;

    let data_json: Value = serde_json::from_str(&data)
        .map_err(|e| err(&e.to_string()))?;
    validate_content(&EXPRESSION_DATA_SCHEMA, &data_json)?;

    let expression = Expression {
        data: data_json,
        author,
        timestamp,
        proof,
    };

    let entry_hash = hash_entry(&expression)?;
    let _header_hash = create_entry(&expression)?;

    hc_time_index::index_entry(expression.author.clone(), expression.clone(), LinkTag::new(EXPRESSION_TAG_NAME))
        .map_err(|e| err(&e.to_string()))?;
    
    Ok(entry_hash)
}

#[hdk_extern]
pub fn get_expression_by_author(input: GetByAuthorInput) -> ExternResult<Vec<Expression>> {
    let links = hc_time_index::get_links_for_time_span(
        input.author, input.from, input.until, Some(LinkTag::new(EXPRESSION_TAG_NAME)), None
    ).map_err(|e| err(&e.to_string()))?;
    debug!("Got links: {:#?}", links);
    links.into_iter()
        .map(|link| {
            let element = get(link.target, GetOptions::default())?
                .ok_or(err("Could not get entry after commit."))?;
            let expression = element.entry().to_app_option()?
                .ok_or(err("Could not deserialize element to Expression."))?;
            Ok(expression)
        })
        .collect()
}

#[hdk_extern]
pub fn get_expression_by_address(input: EntryHash) -> ExternResult<Option<Expression>> {
    let optional_element = get(input, GetOptions::default())?;
    if let Some(element) = optional_element {
        let expression: Expression = element.entry()
            .to_app_option()?
            .ok_or(err("Could not deserialize element to Expression."))?;
        
        return Ok(Some(expression))
    }

    Ok(None)
}

#[hdk_extern]
pub fn recv_private_expression(input: PrivateExpression) -> ExternResult<EntryHash> {
    let agent = PrivateAcaiAgent(input.author.clone());
    let agent_entry_hash = hash_entry(&agent)?;
    create_entry(&agent)?;
    
    let expression_entry_hash = hash_entry(&input)?;
    create_entry(&input)?;

    create_link(
        agent_entry_hash,
        expression_entry_hash.clone(),
        LinkTag::new(EXPRESSION_TAG_NAME),
    )?;

    Ok(expression_entry_hash)
}

#[hdk_extern]
pub fn send_private_expression(input: PrivateExpressionInput) -> ExternResult<PrivateExpression> {
    let ExpressionInput { data, author, timestamp, proof } = input.expression;

    let data_json: Value = serde_json::from_str(&data)
        .map_err(|e| err(&e.to_string()))?;
    validate_content(&EXPRESSION_DATA_SCHEMA, &data_json)?;

    let expression = PrivateExpression {
        data: data_json,
        author,
        timestamp,
        proof,
    };

    // Call the user's remote zome
    // TODO here we want some pattern better than this; only having this succeed when agent is online is not great
    // Here I am sending the identity of the callee of this fn since I dont know if we can get this information in recv_private_expression?
    // I'd imagine there is some way but for now this can work fine...
    call_remote(
        input.to,
        ZomeName::from(ZOME_NAME),
        FunctionName::from(RECV_PRIVATE_EXPRESSION_FUNC_NAME),
        None,
        &expression,
    )?;

    Ok(expression)
}

#[hdk_extern]
pub fn inbox(input: InboxInput) -> ExternResult<Vec<PrivateExpression>> {
    match input.from {
        Some(ident) => {
            let links = get_links(
                hash_entry(&PrivateAcaiAgent(ident.clone()))?,
                Some(LinkTag::new(EXPRESSION_TAG_NAME)),
            )?;

            let experssions = links
                .into_iter()
                .map(|link| {
                    let element = get(link.target, GetOptions::default())?
                        .ok_or(err("Could not get entry after commit."))?;
                    let expression: PrivateExpression = element.entry()
                        .to_app_option()?
                        .ok_or(err("Could not deserialize element to PrivateExpression."))?;

                    Ok(expression)
                })
                .collect::<Result<Vec<PrivateExpression>, WasmError>>()?;

            Ok(experssions)
        },
        None => {
            let exp_entry_def = PrivateExpression::entry_def();
            let elements = query(
                QueryFilter::new().entry_type(EntryType::App(
                    AppEntryType::new(PRIVATE_EXPRESSION_ENTRY_DEF_INDEX.into(), ZOME_INDEX.into(), exp_entry_def.visibility)
                )).include_entries(true)
            )?;

            let expressions = elements.into_iter()
                .map(|elem| {
                    let expression: PrivateExpression = elem.entry()
                        .to_app_option()?
                        .ok_or(err("Could not deserialize element to PrivateExpression."))?;
                    
                    Ok(expression)
                })
                .collect::<Result<Vec<PrivateExpression>, WasmError>>()?;

            Ok(expressions)
        }
    }
}
