extern crate cmake;

fn main() {
    let dst = cmake::Config::new("usrsctp").build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=usrsctp");
}
