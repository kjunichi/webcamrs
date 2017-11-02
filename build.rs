#[cfg(target_os="macos")]
fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/opt/opencv/lib");
}

#[cfg(not(target_os="macos"))]
fn main() {}