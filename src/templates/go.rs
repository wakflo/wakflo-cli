use crate::utils::plugin::{generate_shared_plugin_files, resolve_variables, TaskConfig};
use anyhow::bail;
use convert_case::{Case, Casing};
use loading::Loading;
use std::io::Write;
use std::{fs, path};

pub const MAIN_GO: &str = r#"package main

import (
	"fmt"
	"github.com/wakflo/wakflo-go/sdk/v1"
)

// MainExecute .
func MainExecute(ptr uint32) {
	// Yay!!! you can start hacking
	sdk.ExecutePlugin(func(ctx sdk.TaskContext) sdk.TaskResult {
		fmt.Println("Hello, World!")
		return sdk.TaskResult{}
	})(ptr)
}

// main .
func main() {}
"#;

pub const GO_MOD: &str = r#"module {{ name }}

go 1.20

require github.com/wakflo/wakflo-go v1.0.1 // indirect
"#;

pub(crate) fn create_go_plugin_project(
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
    let mut resolved_str = resolve_variables(GO_MOD, &config)?;
    let mut file = fs::File::create(path::Path::new(format!("{}/go.mod", dir_name).as_str()))?;
    file.write_all(resolved_str.as_bytes())?;

    // generate cargo toml
    fs::create_dir(path::Path::new(format!("{}/tests", dir_name).as_str()))?;

    resolved_str = resolve_variables(MAIN_GO, &config)?;
    file = fs::File::create(path::Path::new(
        format!("{}/main.go", dir_name).as_str(),
    ))?;
    file.write_all(resolved_str.as_bytes())?;

    loading.success("created plugin successfully   ✅");
    Ok(())
}
