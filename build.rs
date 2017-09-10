#[cfg(target_os="macos")]
fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/opt/opencv3/lib");
}

#[cfg(not(target_os="macos"))]
fn main() {}