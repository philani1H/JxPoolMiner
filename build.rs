use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    
    println!("cargo:rerun-if-changed=build.rs");
    
    match target_os.as_str() {
        "windows" => {
            println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");
            println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
        }
        "macos" => {
            println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.13");
        }
        _ => {}
    }
}
