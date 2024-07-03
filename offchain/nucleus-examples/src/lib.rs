#[no_mangle]
pub fn init() {}

#[no_mangle]
pub fn post1(v: u32) -> u32 {
    v + 1
}
