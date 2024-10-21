fn main() {
    println!("cargo:rustc-link-lib=dylib=SDL2");

    println!("cargo:rustc-link-search=native=./libs");
}
