extern crate cc;

#[cfg(target_os="macos")]
fn main() {
    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .flag("-std=c++11")
        .file("src/webcam/cpp/src/webcam.cpp")
        .include("src/webcam/cpp/include")
        .include("/usr/local/opt/opencv/include/opencv4")
        .compile("libwebcam.a");

    println!("cargo:rustc-link-search=native=/usr/local/opt/opencv/lib");
}

#[cfg(not(target_os="macos"))]
fn main() {
    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .file("src\\webcam\\cpp\\src\\webcam.cpp")
        .include("src\\webcam\\cpp\\include")
        .include("C:\\tools\\opencv\\build\\include")
        .compile("libwebcam.a");
}
