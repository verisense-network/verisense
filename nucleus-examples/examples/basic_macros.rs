// use vrs_core_macros::{get, init, post};
use vrs_core_sdk::{
    codec::{Decode, Encode},
    get, init, post, storage,
};

#[derive(Debug, Decode, Encode)]
pub struct E {
    pub a: Vec<u32>,
    pub b: i32,
    pub c: u32,
}

#[derive(Debug, Decode, Encode)]
pub struct D {
    pub b: i32,
}

#[init]
pub fn init(e: E, u: u32) {
    let b = format!("b = {}, c = {}", e.b, e.c);
    storage::put(&u.to_be_bytes(), b.as_bytes()).unwrap();
    vrs_core_sdk::println!("nucleus id: {:?}", vrs_core_sdk::nucleus_id());
}

#[post]
pub fn cc(a: String, b: String) -> Result<String, String> {
    // cross char in a and char in b to  gernerate c
    if a.len() != b.len() {
        return Err("a and b should have the same length".to_string());
    }
    let mut c = String::new();
    let mut a_iter = a.chars();
    let mut b_iter = b.chars();
    loop {
        match (a_iter.next(), b_iter.next()) {
            (Some(a), Some(b)) => {
                c.push(a);
                c.push(b);
            }
            _ => {
                break;
            }
        }
    }
    Ok(c)
}

#[post]
pub fn use_codec(d: D) -> Result<E, String> {
    Ok(E {
        a: vec![d.b as u32],
        b: d.b,
        c: 0,
    })
}

#[get]
pub fn should_not_call_put() -> Result<(), String> {
    let vec = vec![0u8; 65536 * 4];
    storage::put(b"aaaaaaaaaaaaaaaaaaaaa", &vec).map_err(|e| e.to_string())
}

#[post]
pub fn should_call_put() -> Result<(), String> {
    let vec = vec![0u8; 65536 * 4];
    storage::put(b"bbbbbbbbbbbbbbbbbbbbb", &vec).map_err(|e| e.to_string())
}
