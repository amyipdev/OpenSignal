use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("{}", env::var("OUT_DIR").unwrap());

    std::process::Command::new("pwd").spawn();
    std::process::Command::new("ls").spawn();

    glib_build_tools::compile_resources(
        &["./icons"],                            // Search path for XML and resources
        "./icons.gresource.xml",            // The XML file
        &out_dir.join("icons.gresource").into_os_string().to_str().unwrap(), // Output path
    );

    println!("cargo:rerun-if-changed=icons.gresource.xml");
    println!("cargo:rerun-if-changed=icons/icon1.png");
    println!("cargo:rerun-if-changed=icons/icon2.png");
}
