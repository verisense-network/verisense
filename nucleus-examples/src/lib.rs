use std::default;

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

// #[post]
// pub fn post(e: E, s: String) -> Result<D, ()> {
//     storage::put(b"hello", b"world").map_err(|_| ())?;
//     Ok(D { b: 1582 })
// }

#[get]
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
#[post] //i1o1
pub fn i1o1(a: String) -> String {
    a
}
#[post] //i0o0
pub fn i0o0() {}
#[post] //i1o0
pub fn i1o0(a: String) {}

// //1. must put the length of the string before the string
// //2. host must knew the type of input and output
// // preserve two
// #[no_mangle]
// pub fn cross_string_decoded(a: *const u8, b: *const u8) -> *const u8 {
//     fn split(ptr: *const u8) -> (u32, *const u8) {
//         unsafe {
//             let length = u32::from_ne_bytes(*(ptr as *const [u8; 4]));
//             (length, ptr.add(4))
//         }
//     }
//     fn merge(len: u32, a: *const u8) -> *const u8 {
//         unsafe {
//             let mut v = Vec::with_capacity(4 + len as usize);
//             v.extend_from_slice(&len.to_ne_bytes());
//             v.extend_from_slice(std::slice::from_raw_parts(a, len as usize));
//             let p = v.as_ptr();
//             std::mem::forget(v);
//             p
//         }
//     }
//     let back = a.clone();
//     let mut bytes_a = unsafe { std::slice::from_raw_parts(a, 100) };
//     println!("{:?}", bytes_a);
//     // cross char in a and char in b to  gernerate c
//     let (lena, a) = split(a);
//     let (lenb, b) = split(b);
//     println!("lena: {:?}, lenb: {:?}", lena, lenb);
//     let mut bytes_a = unsafe { std::slice::from_raw_parts(a, lena as usize) };
//     println!("bytes_a: {:?}", bytes_a);
//     let decoded_a = <String as codec::Decode>::decode(&mut bytes_a).unwrap();

//     let mut bytes_b = unsafe { std::slice::from_raw_parts(b, lenb as usize) };
//     let decoded_b = <String as codec::Decode>::decode(&mut bytes_b).unwrap();

//     let a = decoded_a;
//     let b = decoded_b;
//     if a.len() != b.len() {
//         let new_c: Vec<u8> = <Result<String, String> as codec::Encode>::encode(&Err(
//             "a and b should have the same length".to_string(),
//         ));

//         return merge(new_c.len() as u32, new_c.as_ptr());
//     };
//     let mut c = String::new();
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
//     //encdoe string to bytes
//     let new_c = <Result<String, String> as codec::Encode>::encode(&Ok(c));
//     let p = new_c.as_ptr();
//     merge(new_c.len() as u32, p)
//     // back
// }
#[get]
pub fn get() -> i32 {
    5
}
#[test]
pub fn test_cross_string() {
    use codec::{Decode, Encode};

    #[derive(Debug, Decode, Encode)]
    struct B {
        pub s: String,
        pub c: String,
    }
    #[derive(Debug, Decode, Encode)]
    struct A {
        pub s: String,
        pub b: B,
        pub c: String,
    }
    let new_c = <A as codec::Encode>::encode(&A {
        s: "1".to_string(),
        b: B {
            s: "2".to_string(),
            c: "3".to_string(),
        },
        c: "1".to_string(),
    });
    // let new_c = <Result<u8, u8> as codec::Encode>::encode(&Err(1));
    println!("new_c: {:?}", new_c);
    let mut c = String::new();
    let a = "111111".to_owned();
    let b = "222222".to_owned();
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
    println!("c: {:?}", c);
    let encode_a = <String as codec::Encode>::encode(&a);
    // insert length before encode_a as ne bytes
    let mut v = Vec::with_capacity(4 + encode_a.len());
    v.extend_from_slice(&(encode_a.len() as u32).to_ne_bytes());
    v.extend_from_slice(encode_a.as_slice());
    println!("v: {:?}", v);
    let encode_b = <String as codec::Encode>::encode(&b);
    let mut t = Vec::with_capacity(4 + encode_b.len());
    t.extend_from_slice(&(encode_b.len() as u32).to_ne_bytes());
    t.extend_from_slice(&encode_b.as_slice());
    let r = cross_string_decoded(v.as_ptr(), t.as_ptr());
    // pointer to vec
    let mut bytes = unsafe { std::slice::from_raw_parts(r, 4 + 14) };
    println!("bytes: {:?}", bytes);
}
