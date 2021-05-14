use std::fs::read_dir;
use std::error::Error as StdError;
use std::env;
use std::path::PathBuf;

use cc::Build;

fn main() -> Result<(), Box<dyn StdError + Send + Sync + 'static>> {
    const CPROC_SRC_PATH: &str = "./src/cproc";

    // read target source and header file lists from ./src/cproc directory
    let mut cproc_src_files = Vec::new();
    for files in read_dir(CPROC_SRC_PATH)? {
        let filename = files?.file_name().to_str().unwrap().to_string();
        let filepath = format!("{}/{}", CPROC_SRC_PATH, filename);

        if filename.contains(".c") {
            cproc_src_files.push(PathBuf::from(&filepath));
        }

        // re-compile if source file changed.
        println!("cargo:rerun-if-changed={}", filepath);
    }

    // create cc context and compile it. (emit into rustc's OUT_DIR)
    let mut cc = Build::new();
    cc.files(&cproc_src_files);
    cc.include(CPROC_SRC_PATH);
    cc.static_flag(true);
    cc.flag("--std=c11");
    cc.flag("-Wall");
    cc.flag("-Wpedantic");
    cc.flag("-Wno-parentheses");
    cc.flag("-g");
    cc.flag("-Wno-switch");
    cc.warnings(false);
    cc.compile("cproc");

    Ok(())
}