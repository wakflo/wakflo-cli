<<<<<<< HEAD
use crate::utils::plugin::{generate_shared_plugin_files, resolve_variables, PluginConfig};
use anyhow::bail;
use convert_case::{Case, Casing};
use loading::Loading;
use std::io::Write;
use std::{fs, path};

pub const MAIN_RS: &str = r#"use serde_json::Value;
use wakflo_sdk::prelude::*;

#[workflow_task]
pub fn execute() -> JsonOutput<String> {
   let msg = "Hello Wakflo";
   Ok(serde_json::Value::Null)
=======
use std::{fs, path};
use std::io::Write;
use anyhow::bail;
use convert_case::{Case, Casing};
use loading::Loading;
use crate::utils::plugin::{PluginConfig, resolve_variables, generate_shared_plugin_files};

pub const MAIN_RS: &str = r#"use serde_json::Value;

pub fn execute() -> anyhow::Result<Value> {
    setup_panic!();

    WakfloCli::init();
>>>>>>> 85173fa (feat: first commit)
}
"#;

pub const CARGO_TOML: &str = r#"[package]
name = "{{ name }}"
version = "0.0.1"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
anyhow = "0.2"
<<<<<<< HEAD
sersde_json = "3.3"
wakflo_sdk = "0.0"
=======
sersde_json= "3.3"
>>>>>>> 85173fa (feat: first commit)
"#;

// pub(crate) fn check_wakflo_project(p: PathBuf) -> anyhow::Result<()> {
//     Ok(())
// }

<<<<<<< HEAD
pub(crate) fn create_rust_plugin_project(
    config: PluginConfig,
    loading: &Loading,
) -> anyhow::Result<()> {
    let dir_name = config.name.to_case(Case::Kebab).to_lowercase();
=======
pub(crate) fn create_rust_plugin_project(config: PluginConfig, loading: &Loading) -> anyhow::Result<()> {
    let dir_name = config.name.clone().to_case(Case::Kebab).to_lowercase();
>>>>>>> 85173fa (feat: first commit)
    let plugin_path = path::Path::new(dir_name.as_str());
    if plugin_path.exists() && plugin_path.is_dir() {
        bail!("dir with plugin name {} already exist", dir_name)
    }

    if let Err(e) = fs::create_dir(plugin_path) {
        bail!("{}", e)
    };

    // generate wakflo plugin config
    generate_shared_plugin_files(&config)?;

    // generate cargo toml
    let mut resolved_str = resolve_variables(CARGO_TOML, &config)?;
    let mut file = fs::File::create(path::Path::new(format!("{}/cargo.toml", dir_name).as_str()))?;
    file.write_all(resolved_str.as_bytes())?;

    // generate cargo toml
    fs::create_dir(path::Path::new(format!("{}/lib", dir_name).as_str()))?;
    fs::create_dir(path::Path::new(format!("{}/tests", dir_name).as_str()))?;

    resolved_str = resolve_variables(MAIN_RS, &config)?;
<<<<<<< HEAD
    file = fs::File::create(path::Path::new(
        format!("{}/lib/main.rs", dir_name).as_str(),
    ))?;
=======
    file = fs::File::create(path::Path::new(format!("{}/lib/main.rs", dir_name).as_str()))?;
>>>>>>> 85173fa (feat: first commit)
    file.write_all(resolved_str.as_bytes())?;

    loading.success("created plugin successfully   ✅");
    Ok(())
<<<<<<< HEAD
}
=======
}
>>>>>>> 85173fa (feat: first commit)
