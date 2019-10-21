use std::env;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search={}/cbash/", project_dir);
    println!("cargo:rustc-link-lib=static=Cbash");
    println!("cargo:rustc-link-lib=static=libboost_iostreams-vc142-mt-x64-1_71");
    println!("cargo:rustc-link-lib=static=zlibstatic");
}
