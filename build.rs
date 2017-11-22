extern crate cmake;

fn main() {
    let dst = cmake::build("libluadns");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=luadns");

    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-lib=luajit-5.1");
}
