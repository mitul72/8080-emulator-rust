use std::env;
use std::path::PathBuf;

fn main() {
    let libs_dir = PathBuf::from("libs");

    // Append it to the PATH environment variable
    let new_path = format!("{};{}", libs_dir.display(), env::var("PATH").unwrap());
    env::set_var("PATH", new_path);

    println!("cargo:rustc-link-lib=dylib=SDL2");

    println!("cargo:rustc-link-search=native=./libs");
}
