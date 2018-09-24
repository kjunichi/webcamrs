#[cfg(target_os="macos")]
fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/opt/opencv/lib");
}

#[cfg(not(target_os="macos"))]
extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .file("src\\webcam\\cpp\\src\\webcam.cpp")
        .include("src\\webcam\\cpp\\include")
        .include("C:\\tools\\opencv\\build\\include")
        .compile("libwebcam.a");
}
