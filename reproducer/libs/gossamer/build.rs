use std::fs;
use std::path::PathBuf;

fn main() {
    let srcdir = PathBuf::from("../glib");
    let glib_dir = fs::canonicalize(&srcdir).unwrap();
    // Build glib library
    // go build -o libglib.a -buildmode=c-archive glib.go
    println!(
        "cargo:rustc-link-search=native={}/libglib.a",
        glib_dir.to_string_lossy()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        glib_dir.to_string_lossy()
    );
    println!("cargo:rustc-link-lib=static=glib");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
