use crate::utils::dir_files::{get_root_dir, setup_build_output};
use crate::utils::types::{PluginLanguage, WakfloExtension};
use loading::Loading;
use std::fs;
use std::process::Command;

pub(crate) fn cargo_build(module: &WakfloExtension, loading: &Loading) -> anyhow::Result<()> {
    let cmd = match module.plugin.language {
        PluginLanguage::Rust => Command::new("cargo")
            // .arg("wasi")
            .arg("build")
            .arg("--release")
            .arg("--target")
            .arg("wasm32-wasi")
            .env("LS_COLORS", "rs=0:di=38;5;27:mh=44;38;5;15")
            .output(),

        PluginLanguage::Golang => {
            // set dist folder
            setup_build_output().expect("not handled");

            // final binary output path
            let dest_path = format!(
                "dist/{}.wasm",
                module.plugin.name.clone()
            );

            Command::new("tinygo")
                .arg("build")
                // .arg("-wasm-abi=generic")
                .arg("-target=wasi")
                .arg("-gc=leaking")
                .arg("-o")
                .arg("main.go")
                .arg(dest_path)
                .env("LS_COLORS", "rs=0:di=38;5;27:mh=44;38;5;15")
                .output()
        },
        _ => anyhow::bail!("Sorry unsupported language")
    };

    let output = cmd.expect("failed to execute process");

    if !output.status.success() {
        anyhow::bail!("error: {}", String::from_utf8_lossy(&output.stderr));
    }

    if module.plugin.language != PluginLanguage::Rust {
        return Ok(())
    }

    // run post process build
    loading.text("Running postprocess ...");
    cargo_build_postprocess(module)?;
    loading.text("Completed postprocess ...");

    Ok(())
}

#[allow(unused)]
pub(crate) fn copy_rust_binary(module: &WakfloExtension) -> anyhow::Result<()> {
    let root_dir = get_root_dir()?;

    match root_dir {
        None => {}
        Some(dir) => {
            let from = dir.clone().join(format!(
                "target/wasm32-wasi/release/{}.wasm",
                module.plugin.name.clone()
            ));
            let to = dir
                .clone()
                .join(format!("dist/{}.wasm", module.plugin.name.clone()));
            let from_d = dir.clone().join(format!(
                "target/wasm32-wasi/release/{}.d",
                module.plugin.name.clone()
            ));
            let to_d = dir
                .clone()
                .join(format!("dist/{}.d", module.plugin.name.clone()));
            fs::copy(from, to)?;
            fs::copy(from_d, to_d)?;
        }
    };

    Ok(())
}

pub(crate) fn cargo_build_postprocess(module: &WakfloExtension) -> anyhow::Result<()> {
    setup_build_output()?;

    // let co = Command::new("wasm-tools")
    //     .arg("component")
    //     .arg("new")
    //     .arg(format!("./target/wasm32-wasi/release/{}.wasi.wasm", module.plugin.name.clone()))
    //     .arg("-o")
    //     .arg(format!("./dist/{}.wasm", module.plugin.name.clone()))
    //     .arg("--adapt")
    //     .arg("./wit/wasi_snapshot_preview1.wasm")
    //     .env("LS_COLORS", "rs=0:di=38;5;27:mh=44;38;5;15")
    //     .output()
    //     .expect("failed to execute process");
    //
    // if !co.status.success() {
    //     anyhow::bail!("error: {}", String::from_utf8_lossy(&co.stderr));
    // }

    match module.plugin.language {
        PluginLanguage::Rust => {
            copy_rust_binary(module)?;
        }
        PluginLanguage::Typescript => {}
        PluginLanguage::Javascript => {}
        PluginLanguage::Golang => {}
        PluginLanguage::Zig => {}
    }

    Ok(())
}

pub(crate) fn cargo_test() {
    let output = Command::new("cargo")
        .arg("test")
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
