//! Stdio module for printing to stdout and stderr of the host environment, useful for debugging on local.
//! It is only available in the host environment, i.e., the environment where the Wasm is running.
//! The functions in this module can be called both in get and post functions.
//!
//! # Examples
//!
//! ```
//! use vrs_core_sdk::post;
//!
//! #[post]
//! pub fn post(name: String) {
//!     vrs_core_sdk::println!("Hello, {}", name);
//!     vrs_core_sdk::eprintln!("Hello, {}", name);
//! }
//! ```

use codec::{Decode, Encode};

#[link(wasm_import_module = "env")]
extern "C" {
    fn stdout_print(ptr: *const u8, len: i32);

    fn stderr_print(ptr: *const u8, len: i32);

    fn get_nucleus_id(ptr: *mut u8);
}

/// Get the id of the current nucleus.
pub fn nucleus_id() -> crate::NucleusId {
    let mut id = crate::NucleusId::from([0u8; 32]).encode();
    unsafe {
        get_nucleus_id(id.as_mut_ptr());
    }
    <crate::NucleusId as Decode>::decode(&mut &id[..]).unwrap()
}

pub fn _print(args: String) {
    let s = <String as Encode>::encode(&args);
    unsafe {
        stdout_print(s.as_ptr(), s.len() as i32);
    }
}

pub fn _eprint(args: String) {
    let s = <String as Encode>::encode(&args);
    unsafe {
        stderr_print(s.as_ptr(), s.len() as i32);
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        let mut f = ::std::format!($($arg)*);
        f.push_str("\n");
        $crate::io::_print(f);
    }};
}

#[macro_export]
macro_rules! print {
    () => {};
    ($($arg:tt)*) => {{
        $crate::io::_print(std::format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! eprintln {
    () => {
        $crate::eprint!("\n")
    };
    ($($arg:tt)*) => {{
        let mut f = ::std::format!($($arg)*);
        f.push_str("\n");
        $crate::io::_eprint(f);
    }};
}

#[macro_export]
macro_rules! eprint {
    () => {};
    ($($arg:tt)*) => {{
        $crate::io::_eprint(std::format!($($arg)*));
    }};
}
