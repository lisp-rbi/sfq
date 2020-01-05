extern crate cc;

// Compile cc library located in ffi
fn main() {
    cc::Build::new()
        .file("src/ffi/ltz.cpp")
        .cpp(true)
        .cpp_link_stdlib("stdc++")
        .compile("liblzt.a");
}
