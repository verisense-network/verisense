use vrs_core_sdk::AccountId;

#[derive(Debug)]
pub struct WasmInfo {
    pub account: AccountId,
    pub name: String,
    pub version: u32,
    pub code: WasmCodeRef,
}

#[derive(Debug)]
pub enum WasmCodeRef {
    File(String),
    Blob(Vec<u8>),
}
