use std::fs;
use std::path::PathBuf;

fn main() {
    let srcdir = PathBuf::from("../gfuzz");
    let gossamer_fuzz_dir = fs::canonicalize(&srcdir).unwrap();
    // Build gossamer_fuzz library
    // go build -o libgfuzz.a -buildmode=c-archive gfuzz.go
    println!(
        "cargo:rustc-link-search=native={}/libgfuzz.a",
        gossamer_fuzz_dir.to_string_lossy()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        gossamer_fuzz_dir.to_string_lossy()
    );
    println!("cargo:rustc-link-lib=static=gfuzz");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
