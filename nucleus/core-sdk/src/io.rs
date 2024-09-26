use codec::Encode;

#[link(wasm_import_module = "env")]
extern "C" {
    fn stdout_print(ptr: *const u8, len: i32);

    fn stderr_print(ptr: *const u8, len: i32);
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
