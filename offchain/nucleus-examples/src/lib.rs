use codec::Decode;
use nucleus_core::init;

pub mod t {
    use super::*;
    #[derive(Debug, Decode)]
    pub struct E {
        pub a: Vec<u32>,
        pub b: i32,
        pub c: u32,
    }
}
// #[init]
// pub fn init() {}

#[init]
pub fn init_a(e: crate::t::E, u: u32) {
    nucleus_core::storage::put(b"hello", b"world").unwrap();
}

// #[init]
// pub fn init_b(e: E) {
//     nucleus_core::storage::put(b"hello", b"world").unwrap();
// }

#[no_mangle]
pub fn post(v: Vec<u8>) -> u32 {
    let mut r = v.as_ref();
    let a = t::E::decode(&mut r).unwrap();
    nucleus_core::storage::put(b"hello", b"world").unwrap();
    return 3;
}

pub fn __init(args: ()) {}

pub fn __init_a(args: (t::E, u32)) {
    fn _test(a: ()) {}
    _test(())
}
