use vrs_primitives::AccountId;

#[derive(Debug)]
pub struct WasmInfo {
    pub account: AccountId,
    pub name: String,
    pub version: u32,
    pub code: WasmCodeRef,
}

#[derive(Debug)]
pub enum WasmCodeRef {
    File(std::path::PathBuf),
    Blob(Vec<u8>),
}
