use std::default;

use t::{D, E};
use vrs_core_macros::{get, init, post};
use vrs_core_sdk::{set_timer, storage};

pub mod t {
    // use vrs_core_sdk::codec::{Decode, Encode};
    use parity_scale_codec::{Decode, Encode};

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
            default => {
                break;
            }
        }
    }
    Ok(c)
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
#[post]
pub fn test_put_get_static() -> Result<String, String> {
    storage::put(b"test", b"test_value").map_err(|e| e.to_string())?;
    let res = storage::get_static(b"test")
        .map_err(|e| e.to_string())?
        .unwrap();
    let s = String::from_utf8(res).unwrap();
    if s != "test_value" {
        return Err("test_value not equal".to_string());
    }

    // let s = format!("{:?}", res.len());
    Ok(s)
}
#[post]
pub fn test_put_get() -> Result<String, String> {
    let long_string = "1".repeat(65536 * 16);

    storage::put(b"test", long_string.as_bytes()).map_err(|e| e.to_string())?;
    let res = storage::get(b"test").map_err(|e| e.to_string())?.unwrap();
    let s = String::from_utf8(res).unwrap();
    if s != long_string {
        return Err("test_value not equal".to_string());
    }
    Ok(s)
}
#[post]
pub fn test_get_not_found() -> Result<String, String> {
    assert!(storage::get(b"test").map_err(|e| e.to_string())?.is_none());
    Ok("".to_owned())
}

#[post] //i1o1
pub fn i1o1(a: String) -> String {
    a
}
#[post] //i0o0
pub fn i0o0() {}
#[post] //i1o0
pub fn i1o0(a: String) {}
#[post]
pub fn i0o1() -> String {
    "123".to_string()
}
#[get]
pub fn get() -> i32 {
    5
}
#[get]
pub fn get_data(key: String) -> Result<String, String> {
    let res = storage::get(key.as_bytes())
        .map_err(|e| e.to_string())?
        .unwrap();
    let s = String::from_utf8(res).unwrap();
    Ok(s)
}
#[post]
pub fn test_delay(a: String, b: i32) {
    storage::put(b"delay", format!("delay_complete {} {}", a, b).as_bytes()).unwrap();
}
#[post]
pub fn test_set_timer() {
    storage::put(b"delay", format!("init").as_bytes());

    let a = "abc".to_string();
    let b = 123;
    set_timer!(4, test_delay, a, b);
}
#[post]
pub fn test_set_tree_mod_timer() {
    test_delay("init".to_string(), 0);
    for i in (1..=10).rev() {
        set_timer!(i, test_delay, "abc".to_string(), i as i32).unwrap();
    }
}

#[post]
pub fn test_stream() -> Result<i32, String> {
    Ok(555)
}
//          0
//        /   \
//       1     2
//    /   \    /  \
//   1     2   1   2
//  / \   / \ / \ / \
// 1   2 1  2 1 2 1 2

//            1
//        /        \
//       2          3
//    /   \       /    \
//   4     5      6     7
//  / \   / \    / \   / \
// 8   9 10  11 12 13 14 15

#[post]
pub fn test_set_perfect_tree_mod_timer(i: i32, using_time: i32) -> Result<i32, String> {
    if i == 1 {
        storage::put(b"delay", format!("node {} using time {}", 1, 0).as_bytes())
            .map_err(|e| e.to_string())?;
    } else {
        storage::put(
            b"delay",
            format!("node {} using time {}", i, using_time).as_bytes(),
        )
        .map_err(|e| e.to_string())?;
    }
    if i >= 8 {
        return Ok(i);
    }
    set_timer!(1, test_set_perfect_tree_mod_timer, i * 2, using_time + 1);
    set_timer!(
        2,
        test_set_perfect_tree_mod_timer,
        i * 2 + 1,
        using_time + 2
    );

    Ok(i)
}

// #[test]
// pub fn test_cross_string() {
//     use codec::{Decode, Encode};

//     #[derive(Debug, Decode, Encode)]
//     struct B {
//         pub s: String,
//         pub c: String,
//     }
//     #[derive(Debug, Decode, Encode)]
//     struct A {
//         pub s: String,
//         pub b: B,
//         pub c: String,
//     }
//     let new_c = <A as codec::Encode>::encode(&A {
//         s: "1".to_string(),
//         b: B {
//             s: "2".to_string(),
//             c: "3".to_string(),
//         },
//         c: "1".to_string(),
//     });
//     // let new_c = <Result<u8, u8> as codec::Encode>::encode(&Err(1));
//     println!("new_c: {:?}", new_c);
//     let mut c = String::new();
//     let a = "111111".to_owned();
//     let b = "222222".to_owned();
//     println!(
//         "{}{}",
//         hex::encode(<String>::encode(&a)),
//         hex::encode(<String>::encode(&b))
//     );
//     assert!(false);
//     let mut a_iter = a.chars();
//     let mut b_iter = b.chars();
//     loop {
//         match (a_iter.next(), b_iter.next()) {
//             (Some(a), Some(b)) => {
//                 c.push(a);
//                 c.push(b);
//             }
//             default => {
//                 break;
//             }
//         }
//     }
//     println!("c: {:?}", c);
//     let encode_a = <String as codec::Encode>::encode(&a);
//     // insert length before encode_a as ne bytes
//     let mut v = Vec::with_capacity(4 + encode_a.len());
//     v.extend_from_slice(&(encode_a.len() as u32).to_ne_bytes());
//     v.extend_from_slice(encode_a.as_slice());
//     println!("v: {:?}", v);
//     let encode_b = <String as codec::Encode>::encode(&b);
//     let mut t = Vec::with_capacity(4 + encode_b.len());
//     t.extend_from_slice(&(encode_b.len() as u32).to_ne_bytes());
//     t.extend_from_slice(&encode_b.as_slice());
//     // let r = cross_string_decoded(v.as_ptr(), t.as_ptr());
//     // pointer to vec
//     // let mut bytes = unsafe { std::slice::from_raw_parts(r, 4 + 14) };
//     // println!("bytes: {:?}", bytes);
// }
