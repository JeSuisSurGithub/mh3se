use std::env;
use std::fs;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        return;
    }

    println!("cargo:rerun-if-changed=mh3se.ico");
    println!("cargo:rerun-if-env-changed=BUILD_INST");

    let out = env::var("OUT_DIR").unwrap();


    match env::var("BUILD_INST").as_deref() {
        Ok("cli") => {
            {
                println!("cargo:warning=Compiling CLI resources!");
                let mut res = winres::WindowsResource::new();
                res.set_windres_path("x86_64-w64-mingw32-windres");
                res.set_icon("mh3se.ico");
                res.set("ProductName", "MH3 Save Editor CLI");
                res.set("FileDescription", "Monster Hunter 3 Save Editor CLI");
                res.compile().unwrap();

                fs::rename(
                    format!("{}/resource.o", out),
                    format!("{}/cli-resource.o", out),
                ).unwrap();
            }
            println!("cargo:rustc-link-arg={}/cli-resource.o", out);
        }
        Ok("webui") => {
            println!("cargo:warning=Compiling WEBUI resources!");
            {
                let mut res = winres::WindowsResource::new();
                res.set_windres_path("x86_64-w64-mingw32-windres");
                res.set_icon("mh3se.ico");
                res.set("ProductName", "MH3 Save Editor WEBUI");
                res.set("FileDescription", "MH3 Save Editor WEBUI");
                res.compile().unwrap();

                fs::rename(
                    format!("{}/resource.o", out),
                    format!("{}/webui-resource.o", out),
                ).unwrap();
            }
            println!("cargo:rustc-link-arg={}/webui-resource.o", out);
        }
        _ => {}
    }
}