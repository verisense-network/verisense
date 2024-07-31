use t::{D, E};
use vrs_core_sdk::{get, init, post, storage};

pub mod t {
    use codec::{Decode, Encode};

    #[derive(Debug, Decode)]
    pub struct E {
        pub a: Vec<u32>,
        pub b: i32,
        pub c: u32,
    }

    #[derive(Debug, Encode)]
    pub struct D {
        pub b: i32,
    }
}

#[init]
pub fn init(e: t::E, u: u32) {
    storage::put(b"hello", b"world").unwrap();
}

#[post]
pub fn post(e: E) -> Result<D, ()> {
    // let a = storage::get(b"hello").unwrap();
    // let b = storage::get(b"world").unwrap();

    storage::put(b"hello", b"world").map_err(|_| ())?;
    Ok(D { b: 1 })
}

#[post("postOne")]
pub fn post1(e: E) {
    storage::put(b"hello", b"world").unwrap();
}

#[get]
pub fn get(e: E) -> D {
    storage::put(b"hello", b"world").unwrap();
    D { b: 1 }
}
