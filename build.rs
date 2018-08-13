#[cfg(feature = "optimize-cache")]
extern crate cache_size;

use std::env;
use std::fs;

fn generate_cache_const() {
    // Either get l1 cache size or default to 32 KiB
    #[cfg(feature = "optimize-cache")]
    let cache = cache_size::l1_cache_size().unwrap();
    #[cfg(not(feature = "optimize-cache"))]
    let cache = 32768_usize;
    // Generate the code line
    let const_line = format!("pub const CACHE_SIZE: usize = {};", cache).to_string();
    // Generate output file path
    let out = format!("{}/cache_size.rs", env::var("OUT_DIR").unwrap());
    // Write out code
    fs::write(out, const_line).unwrap();
}

fn main() {
    // Generate CACHE_SIZE const and lib.
    generate_cache_const();
}
