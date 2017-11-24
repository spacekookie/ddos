extern crate cmake;

fn main() {
    let dst = cmake::build("libddos");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=ddos");
}
