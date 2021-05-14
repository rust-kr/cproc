use std::env::args;
use std::ffi::CString;

extern "C" {
    fn cproc_main(args: i32, argv: *const *const i8) -> i32;
}

fn main() {
    let argv: Vec<CString> = args().map(|a| CString::new(a).unwrap()).collect();
    let argv: Vec<*const i8> = argv.iter().map(|a| a.as_ptr()).collect();
    let args = argv.len();

    unsafe { cproc_main(args as i32, argv.as_ptr()); }
}
