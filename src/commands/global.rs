use crate::api::make_api;
use crate::utils::build::{cargo_build, cargo_test, go_test};
use crate::utils::dir_files::{fetch_wakflo_plugin_config, get_bundle_plugin_paths};
use clap::Subcommand;
use loading::Loading;
use crate::utils::types::{PluginLanguage};

#[derive(Subcommand)]
pub enum GlobalCommand {
    Publish,
    Test,
    Run,
    Build,
}

impl GlobalCommand {
    pub fn run_plugin() -> anyhow::Result<()> {
        Ok(())
    }

    pub fn build_plugin() -> anyhow::Result<()> {
        let wakflo = fetch_wakflo_plugin_config().expect("failed to find wakflo plugin config");
        if wakflo.is_none() {
            return Ok(());
        }

        let loading = Loading::default();
        loading.text("Compiling plugin ...");

        // run cargo build
        match cargo_build(&wakflo.unwrap(), &loading) {
            Ok(_) => loading.success("Compiled binary"),
            Err(e) => {
                loading.fail(e);
                loading.end();
                panic!("")
            }
        };

        loading.success("Compiled successfully");
        loading.end();
        Ok(())
    }

    pub fn test_plugin() -> anyhow::Result<()> {
        let wakflo = fetch_wakflo_plugin_config().expect("failed to find wakflo plugin config");
        if wakflo.is_none() {
            return Ok(());
        }

        match wakflo.expect("missing wakflo workspace").plugin.language {
            PluginLanguage::Rust => {
                // run cargo test
                cargo_test();
            }
            PluginLanguage::Golang => {
                // run go test
                go_test();
            }
            _ => {}
        };

        Ok(())
    }

    pub fn deploy_plugin() -> anyhow::Result<()> {
        let loading = &Loading::default();
        loading.text("preparing metadata...");
        let wakflo_option =
            fetch_wakflo_plugin_config().expect("failed to find wakflo plugin config");
        if wakflo_option.is_none() {
            loading.fail("invalid wakflo project");
            loading.end();
            return Ok(());
        }

        loading.text("validating plugin...");
        let wakflo = wakflo_option.unwrap();
        let version = wakflo.plugin.version.clone();
        let name = wakflo.plugin.name.clone();
        let plugin_path = get_bundle_plugin_paths().expect("missing plugin");

        loading.text("Compiling plugin ...");
        // run cargo build
        match cargo_build(&wakflo, loading) {
            Ok(_) => loading.success("Compiled binary"),
            Err(e) => {
                loading.fail(e);
                loading.end();
                panic!("")
            }
        };

        loading.text("uploading plugin...");
        match make_api().upload.upload_file(wakflo, plugin_path) {
            Ok(_) => {
                loading.success(format!(
                    "{name} plugin with version {version} deployed successfully"
                ));
            }
            Err(e) => {
                loading.fail(e.to_string());
            }
        };
        loading.end();
        Ok(())
    }
}
