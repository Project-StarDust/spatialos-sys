extern crate bindgen;

use serde::Serialize;
use std::env;
use std::io::ErrorKind;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize)]
struct Package {
    pub name: String,
}

#[derive(Serialize)]
struct Target {
    pub path: String,
    pub r#type: String,
    pub packages: Vec<Package>,
}

#[derive(Serialize)]
struct Targets {
    pub targets: Vec<Target>,
}

fn generate_c_headers_package<P: AsRef<Path>>(path: P) -> Target {
    Target {
        path: path
            .as_ref()
            .join("dependencies/worker_sdk/headers")
            .to_str()
            .expect("Can't convert path to string")
            .to_string(),
        r#type: "worker_sdk".to_string(),
        packages: vec![Package {
            name: "c_headers".to_string(),
        }],
    }
}

fn generate_lib_package_windows<P: AsRef<Path>>(path: P) -> Target {
    let lib_path = path
        .as_ref()
        .join("dependencies/worker_sdk/lib/windows")
        .to_str()
        .expect("Can't convert path to string")
        .to_string();
    println!("cargo:rustc-link-search={}", lib_path);
    println!("cargo:rustc-link-lib=zlibstatic");
    println!("cargo:rustc-link-lib=improbable_worker");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=RakNetLibStatic");
    Target {
        path: lib_path,
        r#type: "worker_sdk".to_string(),
        packages: vec![Package {
            name: "c-static-x86_64-vc141_md-win32".to_string(),
        }],
    }
}

fn generate_lib_package_macos<P: AsRef<Path>>(path: P) -> Target {
    let lib_path = path
        .as_ref()
        .join("dependencies/worker_sdk/lib/windows")
        .to_str()
        .expect("Can't convert path to string")
        .to_string();
    println!("cargo:rustc-link-search={}", lib_path);
    println!("cargo:rustc-link-lib=zlibstatic");
    println!("cargo:rustc-link-lib=improbable_worker");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=RakNetLibStatic");
    Target {
        path: lib_path,
        r#type: "worker_sdk".to_string(),
        packages: vec![Package {
            name: "c-static-x86_64-clang-macos".to_string(),
        }],
    }
}

fn generate_lib_package_linux<P: AsRef<Path>>(path: P) -> Target {
    let lib_path = path
        .as_ref()
        .join("dependencies/worker_sdk/lib/linux")
        .to_str()
        .expect("Can't convert path to string")
        .to_string();
    println!("cargo:rustc-link-search={}", lib_path);
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=improbable_worker");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=RakNetLibStatic");
    println!("cargo:rustc-link-lib=stdc++");
    Target {
        path: lib_path,
        r#type: "worker_sdk".to_string(),
        packages: vec![Package {
            name: "c-static-x86_64-gcc510_pic-linux".to_string(),
        }],
    }
}

fn generate_lib_package<P: AsRef<Path>>(path: P) -> Target {
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("linux") | Ok("android") => generate_lib_package_linux(path),
        Ok("openbsd") | Ok("bitrig") | Ok("netbsd") | Ok("macos") | Ok("ios") => {
            generate_lib_package_macos(path)
        }
        Ok("windows") => generate_lib_package_windows(path),
        tos => panic!("Unknown OS: {:?}!", tos),
    }
}

fn get_targets<P: AsRef<Path> + Clone>(path: P) -> Targets {
    Targets {
        targets: vec![
            generate_c_headers_package(path.clone()),
            generate_lib_package(path),
        ],
    }
}

fn write_packages<P: AsRef<Path> + Clone>(path: P) -> std::io::Result<()> {
    let serialized = serde_json::to_string(&get_targets(path.clone())).unwrap();
    let mut file = std::fs::File::create(path.as_ref().join("worker_packages.json"))?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

fn get_packages<P: AsRef<Path> + Clone>(path: P) -> std::io::Result<std::process::Output> {
    write_packages(path.clone())?;
    Command::new("spatial")
        .arg("package")
        .arg("unpack")
        .arg("--log_directory")
        .arg(path.clone().as_ref().join("logs"))
        .arg("--package_file")
        .arg(path.as_ref().join("worker_packages.json"))
        .output()
}

fn generate_bindings<P: AsRef<Path> + Clone>(path: P, target: &str) {
    if let Err(e) = get_packages(path.clone()) {
        if let ErrorKind::NotFound = e.kind() {
            panic!("`spatial` was not found. Check your PATH.")
        } else {
            panic!("An error occured: {:?}", e)
        }
    } else {
        let target = format!("--target={}", target);
        let bindings = bindgen::Builder::default()
            .header(
                path.clone()
                    .as_ref()
                    .join("dependencies/worker_sdk/headers/include/improbable/c_schema.h")
                    .to_str()
                    .expect("Can't convert path to utf-8")
                    .to_string(),
            )
            .header(
                path.clone()
                    .as_ref()
                    .join("dependencies/worker_sdk/headers/include/improbable/c_worker.h")
                    .to_str()
                    .expect("Can't convert path to utf-8")
                    .to_string(),
            )
            .header(
                path.clone()
                    .as_ref()
                    .join("dependencies/worker_sdk/headers/include/improbable/ngrpc.h")
                    .to_str()
                    .expect("Can't convert path to utf-8")
                    .to_string(),
            )
            .clang_args(&[
                &target
            ])
            .rustified_enum("*")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");
        bindings
            .write_to_file(path.as_ref().join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}

fn main() {
    let target = std::env::var("TARGET").expect("Unable to get target");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    generate_bindings(out_path, &target);
}
