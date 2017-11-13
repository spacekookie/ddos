extern crate cmake;
// extern crate cc;

fn main() {
    let dst = cmake::build("libluadns");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=luadns");
    
    // cc::Build::new()
        // .warnings(false)
        // .file("libluadns/luadnsd.c")
        // .include("/usr/include/luajit-2.0")
        // .flag("-lluajit-5.1")
        // .compile("luadns.so");
}
