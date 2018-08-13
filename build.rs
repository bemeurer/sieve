#[cfg(feature = "optimize-cache")]
extern crate cache_size;

use std::env;
use std::fs;

fn generate_cache_const() {
    #[cfg(feature = "optimize-cache")]
    let cache = cache_size::l1_cache_size().unwrap();
    #[cfg(not(feature = "optimize-cache"))]
    let cache = 32768_usize;
    let const_line = format!("pub const CACHE_SIZE: usize = {};", cache).to_string();

    let out = format!("{}/cache_size.rs", env::var("OUT_DIR").unwrap());
    fs::write(out, const_line).unwrap();
}

fn main() {
    generate_cache_const();
}
