use hdk::prelude::*;
use lazy_static::lazy_static;
use serde_json::Value;

#[derive(SerializedBytes, Serialize, Deserialize, Debug)]
pub struct Properties {
    pub expression_data_schema: String,
}

lazy_static! {
    pub static ref PROPERTIES: Properties = {
        let host_dna_config = dna_info()
            .expect("Could not get zome configuration.")
            .properties;
        Properties::try_from(host_dna_config)
            .expect("Could not convert zome dna properties to Properties.")
    };

    pub static ref EXPRESSION_DATA_SCHEMA: Value = {
        serde_json::from_str(&PROPERTIES.expression_data_schema)
            .expect("Could not convert data string to JSON Value.")
    };
}
