fn main() {
    #[cfg(any(target_os = "openbsd", target_os = "freebsd"))]
    println!(r"cargo:rustc-link-search=/usr/local/lib");

    #[cfg(all(target_os = "macos", feature = "use_mac_framework"))]
    println!("cargo:rustc-link-search=framework=/Library/Frameworks")
}
