#[no_mangle]
pub fn init() {}

#[no_mangle]
pub fn post(v: u32) -> u32 {
    nucleus_core::storage::put(b"hello", b"world").unwrap();
    v + 1
}
