use crate::utils::plugin::{generate_shared_plugin_files, resolve_variables, TaskConfig};
use anyhow::bail;
use convert_case::{Case, Casing};
use loading::Loading;
use std::io::Write;
use std::{fs, path};

pub const MAIN_RS: &str = r#"use wakflo_sdk::prelude::*;

/// {{ name }}
///
/// some description of your task
use wakflo_sdk::prelude::*;
use std::collections::HashMap;

#[wakflo_plugin]
fn execute(ctx: TaskContext) -> TaskResult {
    println!("Hello World");

    let output = RunOutput{
        output: HashMap::new(),
        errors: vec![SystemActivityLog::default()]
    };
    task_json_response(output)
}


#[cfg(test)]
mod test {
   #[test]
   fn it_returns_hello_world() {
      unimplemented!();
   }
}
"#;

pub const CARGO_TOML: &str = r#"[package]
name = "{{ name }}"
version = "0.0.1"
edition = "2021"

[lib]
crate-type = ["cdylib"]
path = "src/lib.rs"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
wakflo_sdk = "0.0"
"#;

// pub(crate) fn check_wakflo_project(p: PathBuf) -> anyhow::Result<()> {
//     Ok(())
// }

pub(crate) fn create_rust_plugin_project(
    config: TaskConfig,
    loading: &Loading,
) -> anyhow::Result<()> {
    let dir_name = config.name.to_case(Case::Kebab).to_lowercase();
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
    fs::create_dir(path::Path::new(format!("{}/src", dir_name).as_str()))?;
    fs::create_dir(path::Path::new(format!("{}/tests", dir_name).as_str()))?;

    resolved_str = resolve_variables(MAIN_RS, &config)?;
    file = fs::File::create(path::Path::new(
        format!("{}/src/lib.rs", dir_name).as_str(),
    ))?;
    file.write_all(resolved_str.as_bytes())?;

    loading.success("created plugin successfully   ✅");
    Ok(())
}
