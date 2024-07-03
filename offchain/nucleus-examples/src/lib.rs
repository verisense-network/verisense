use nucleus_core::Context;

// #[wasm_bindgen]
// extern "C" {
//     fn init();

//     fn post1(context: &mut Context, v: u32);
// }

// #[wasm_bindgen]
#[no_mangle]
pub fn init() {}

// #[wasm_bindgen]
#[no_mangle]
pub fn post1(v: u32) -> u32 {
    v + 1
}
