// All of this code was wrote by hand, please respect it by following the guideline in contributing.md
use crate::packages::Package;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf, process::exit};
use uzers::{get_user_by_name, os::unix::UserExt};

/// The expected Manifest format
#[derive(Deserialize, Serialize)]
struct Manifest {
    version: usize,
    packages: Vec<String>,
}
/// Function to load all the packages
pub fn get() -> Vec<Package> {
    if let Ok(usr) = env::var("SUDO_USER") {
        let pkg_dir = get_user_by_name(&usr)
            .expect("Failed to get the home directory")
            .home_dir()
            .join(".tackle/packages");
        match pkg_dir.read_dir() {
            Ok(mut dir) => {
                let mut packages: Vec<Package> = Vec::new();
                while let Some(Ok(entry)) = dir.next() {
                    let file = entry
                        .file_name()
                        .to_str()
                        .expect("Failed to get a package")
                        .to_string();
                    if file.ends_with(".json") {
                        packages.push(
                            serde_json::from_str(
                                &fs::read_to_string(pkg_dir.join(file))
                                    .expect("Failed to read a package"),
                            )
                            .expect("Failed to parse a package"),
                        );
                    }
                }
                packages
            }
            Err(_) => setup(pkg_dir),
        }
    } else {
        eprintln!("Failed to get the real user behind sudo");
        exit(1)
    }
}

/// Create the package directory and download default packages from github
fn setup(pkg_dir: PathBuf) -> Vec<Package> {
    fs::create_dir_all(&pkg_dir).expect("Failed to create the package dir");
    let mut packages: Vec<Package> = Vec::new();
    let manifest: Manifest = serde_json::from_str(
        &reqwest::blocking::get(
            "https://raw.githubusercontent.com/NoloxoYT/tackle/refs/heads/packages/manifest.json",
        )
        .expect("Failed to get the manifest")
        .text()
        .expect("Failed to get the manifest"),
    )
    .expect("Failed to parse the manifest");
    println!("Loaded manifest version: {}", &manifest.version);
    for package in manifest.packages {
        let raw_pkg = &reqwest::blocking::get(format!(
            "https://raw.githubusercontent.com/NoloxoYT/tackle/refs/heads/packages/{}.json",
            package
        ))
        .expect(&format!("Failed to get the package {}", package))
        .text()
        .expect(&format!("Failed to get the package {}", package));
        let package: Package = serde_json::from_str(&raw_pkg)
            .expect(&format!("Failed to parse the package {}", package));
        println!("Loaded package: {}", package.name);
        println!("Writing package: {} to disk", package.name);
        fs::write(&pkg_dir.join(&format!("{}.json", package.name)), raw_pkg)
            .expect(&format!("Failed to write : {} to disk", package.name));
        packages.push(package);
    }
    packages
}
