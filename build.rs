use std::env;
use winres;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        return;
    }

    println!("cargo:rerun-if-changed=./mh3se.ico");

    let bin_name = env::var("CARGO_BIN_NAME")
        .expect("CARGO_BIN_NAME is not set");

    let mut res = winres::WindowsResource::new();
    res.set_toolkit_path("/usr/bin");
    res.set_windres_path("x86_64-w64-mingw32-windres");
    res.set_icon("mh3se.ico");

    match bin_name.as_str() {
        "mh3se-cli" => {
            res.set("ProductName", "MH3 Save Editor CLI");
            res.set("FileDescription", "Monster Hunter 3 Save Editor CLI");
            res.set("CompanyName", "JeSuisSurGithub");
            res.set("FileVersion", env!("CARGO_PKG_VERSION"));
            res.set("ProductVersion", env!("CARGO_PKG_VERSION"));
        }

        "mh3se-webui" => {
            res.set("ProductName", "MH3 Save Editor WEBUI");
            res.set("FileDescription", "Monster Hunter 3 Save Editor WEBUI");
            res.set("CompanyName", "JeSuisSurGithub");
            res.set("FileVersion", env!("CARGO_PKG_VERSION"));
            res.set("ProductVersion", env!("CARGO_PKG_VERSION"));
        }

        other => {
            println!("cargo:warning=No Windows resource configured for {other}");
            return;
        }
    }
    match res.compile() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{:#?}", e);
            panic!("{e}");
        }
    }
}