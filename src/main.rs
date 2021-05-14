use std::ptr::null;
use std::ffi::CStr;
use std::env::args;

#[no_mangle]
extern fn cproc_main(args: i32, argv: &[*const char]) -> i32;

fn main() {
    let argv: Vec<CStr> = args().for_each(|a| CStr::from(a)).collect();
    let args = argv.len();

    cproc_main(args, argv);
}
