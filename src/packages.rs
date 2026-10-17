// All of this code was wrote by hand, please respect it by following the guideline in contributing.md
pub mod load;
use crate::backend::commands::PackageCommand;
use dashmap::DashMap;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
/// The structure of a package
#[derive(Deserialize, Serialize, Clone)]
pub struct Package {
    pub name: String,
    pub parameters: Vec<Parameters>,
}
/// The struct to 'abstract' a package command
#[derive(Deserialize, Serialize, Clone)]
pub struct Parameters {
    pub param_name: String,
    pub param_type: ParametersType,
}
/// List of the different types an parameter can have
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParametersType {
    Bool {
        command: PackageCommand,
    },
    Choice {
        choices: Vec<String>,
        command: Vec<PackageCommand>,
    },
}

/// Where all the packages will be stored
static PACKAGES: LazyLock<Mutex<DashMap<String, Package>>> =
    LazyLock::new(|| Mutex::new(DashMap::new()));

/// Function to get all supported packages as an Vec<String> for easier render implementation
pub fn get_supported_packages() -> Vec<String> {
    if PACKAGES.lock().is_empty() {
        let packages = load::get();
        *PACKAGES.lock() = DashMap::from(
            packages
                .iter()
                .map(|package| (package.name.to_owned(), package.clone()))
                .collect(),
        );
        packages.iter().map(|pkg| pkg.name.to_owned()).collect()
    } else {
        PACKAGES.lock().iter().map(|kv| kv.name.clone()).collect()
    }
}

/// Function to get all the available Parameter (and their type) for an given package. Return an Vec<Parameters>.
pub fn get_parameters(package: String) -> Option<Vec<Parameters>> {
    if PACKAGES.lock().is_empty() {
        let packages = load::get();
        *PACKAGES.lock() = DashMap::from(
            packages
                .iter()
                .map(|package| (package.name.to_owned(), package.clone()))
                .collect(),
        );
        if let Some(pkg) = PACKAGES.lock().get(&package) {
            Some(pkg.parameters.clone())
        } else {
            None
        }
    } else {
        if let Some(pkg) = PACKAGES.lock().get(&package) {
            Some(pkg.parameters.clone())
        } else {
            None
        }
    }
}

/// Reload packages from disk on every calls
pub fn reload_and_get_packages() -> Vec<String> {
    let packages = load::get();
    *PACKAGES.lock() = DashMap::from(
        packages
            .iter()
            .map(|package| (package.name.to_owned(), package.clone()))
            .collect(),
    );
    get_supported_packages()
}
