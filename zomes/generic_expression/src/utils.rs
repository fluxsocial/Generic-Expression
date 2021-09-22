use hdk::prelude::*;

pub fn err(reason: &str) -> WasmError {
    WasmError::Host(String::from(reason))
}