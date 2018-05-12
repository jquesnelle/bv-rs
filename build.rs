extern crate rustc_version;

use rustc_version::{version, Version};

fn main() {
    if version().unwrap() >= Version::parse("1.26.0").unwrap() {
        println!("cargo:rustc-cfg=int_128");
        println!("cargo:rustc-cfg=inclusive_range");
    }
}