use vrs_core_sdk::AccountId;

#[derive(Debug)]
pub struct WasmInfo {
    pub account: AccountId,
    pub name: String,
    pub version: u32,
    pub descriptor: WasmDescriptor,
    pub code: WasmCodeRef,
}

#[derive(Debug)]
pub enum WasmCodeRef {
    File(String),
    Blob(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct WasmDescriptor {
    pub init: FunctionDescriptor,
    pub get: Vec<FunctionDescriptor>,
    pub post: Vec<FunctionDescriptor>,
}

#[derive(Debug, Clone)]
pub struct FunctionDescriptor {
    pub name: String,
    pub signature: String,
}
