fn main() {
    println!("cargo:rustc-check-cfg=cfg(hide_console)");
    let version = env!("CARGO_PKG_VERSION");
    if !version.contains("alpha") && !version.contains("beta") {
        println!("cargo:rustc-cfg=hide_console");
    }
}
