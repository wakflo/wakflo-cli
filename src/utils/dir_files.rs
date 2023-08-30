use crate::utils::types::{WakfloConfig, WakfloExtension};
use std::{env, fs};
use std::fs::DirEntry;
use std::io::Write;
use std::path::PathBuf;
use loading::Loading;

pub(crate) fn setup_wakflo_dir() -> anyhow::Result<()> {
    let home_dir = dirs::home_dir().expect("can't read home path");
    let mut wakflo_dir = home_dir;
    wakflo_dir.extend(&[".wakflo"]);

    if !wakflo_dir.try_exists()? && !wakflo_dir.is_dir() {
        fs::create_dir(wakflo_dir.clone())?;
    }

    let mut wakflo_config_dir = wakflo_dir.clone();
    wakflo_config_dir.extend(&["config.json"]);

    if !wakflo_config_dir.try_exists()? && !wakflo_config_dir.is_file() {
        let value = serde_json::to_value(WakfloConfig::default())?;
        let json = serde_json::to_string_pretty(&value)?;
        let mut file = fs::File::create(wakflo_config_dir)?;
        file.write_all(json.as_bytes())?;
    }
    Ok(())
}

pub(crate) fn setup_build_output() -> anyhow::Result<()> {
    let root_dir = get_root_dir()?;
    if root_dir.is_none() {
       return  Ok(())
    }

    let mut current_dir = root_dir.expect("");
    current_dir.extend(&["dist"]);

    if !current_dir.try_exists()? && !current_dir.is_dir() {
        fs::create_dir(current_dir.clone())?;
    }

    Ok(())
}

pub(crate) fn get_wakflo_config() -> anyhow::Result<WakfloConfig> {
    let home_dir = dirs::home_dir().expect("can't read home path");
    let mut wakflo_config_dir = home_dir;
    wakflo_config_dir.extend(&[".wakflo/config.json"]);

    if !wakflo_config_dir.try_exists()? && !wakflo_config_dir.is_dir() {
        fs::create_dir(wakflo_config_dir.clone())?;
    }

    let file_bytes = fs::read(wakflo_config_dir)?;
    let c = serde_json::from_slice::<WakfloConfig>(file_bytes.as_slice())?;
    Ok(c)
}

pub(crate) fn update_wakflo_config(config: WakfloConfig) -> anyhow::Result<()> {
    let home_dir = dirs::home_dir().expect("can't read home path");
    let mut wakflo_config_dir = home_dir;
    wakflo_config_dir.extend(&[".wakflo/config.json"]);
    let value = serde_json::to_value(config)?;
    let json = serde_json::to_string_pretty(&value)?;
    let mut file = fs::File::create(wakflo_config_dir)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

const MAX_DIR_SCAN_DEPTH: i8 = 5;

pub(crate) fn get_root_dir() -> anyhow::Result<Option<PathBuf>> {
    let current_dir = env::current_dir().expect("can't read home path");
    let c = _find_root_dir(current_dir, 0)?;
    Ok(c)
}

fn _find_root_dir(dir: PathBuf, mut count: i8) -> anyhow::Result<Option<PathBuf>> {
    let temp_dir = fs::read_dir(dir.clone())?.find(|x| {
        x.as_ref().expect("").file_name().eq("wakflo.toml") && x.clone().as_ref().expect("").file_type().expect("").is_file()
    });

    if let Some(d) = temp_dir {
        let d = d.expect("").path().into_os_string().into_string().expect("");
        return Ok(Some(PathBuf::from(d.replace("wakflo.toml", ""))))
    }

    if count == MAX_DIR_SCAN_DEPTH {
        return Ok(None)
    }

    count = count + 1;
    _find_root_dir(dir.join(".."), count)
}

pub(crate) fn find_wakflo_plugin_file(dir: PathBuf, mut count: i8) -> anyhow::Result<Option<DirEntry>> {
    let temp_dir = fs::read_dir(dir.clone())?.find(|x| {
        x.as_ref().expect("").file_name().eq("wakflo.toml") && x.clone().as_ref().expect("").file_type().expect("").is_file()
    });

    if let Some(d) = temp_dir {
        return Ok(Some(d.expect("")))
    }

    if count == MAX_DIR_SCAN_DEPTH {
        return Ok(None)
    }

    count = count + 1;
    find_wakflo_plugin_file(dir.join(".."), count)
}

pub(crate) fn parse_plugin_file_to_type(file_path: DirEntry) -> anyhow::Result<WakfloExtension> {
    let file_bytes = fs::read_to_string(file_path.path())?;
    Ok(toml::from_str::<WakfloExtension>(file_bytes.as_str())?)
}



pub(crate) fn fetch_wakflo_plugin_config() -> anyhow::Result<Option<WakfloExtension>> {
    let loading = Loading::default();
    let current_dir = env::current_dir().expect("can't read home path");
    let file_path = find_wakflo_plugin_file(current_dir, 0)?;
    if file_path.is_none() {
        loading.fail("failed to find wakflo plugin dir, please run this command in a plugin dir");
        loading.end();
        return Ok(None)
    }

    let wakflo = parse_plugin_file_to_type(file_path.expect(""))?;
    Ok(Some(wakflo))
}

pub(crate) fn get_bundle_plugin_paths() -> anyhow::Result<PathBuf> {
    let dir = env::current_dir()?;
    let files: Vec<PathBuf> = scan_dir::ScanDir::files().read(dir.join("dist"), |iter| {
        iter.filter(|(_, name)| name.ends_with(".wasm"))
            .map(|(entry, _)| entry.path())
            .collect()
    })?;

    if files.is_empty() {
        anyhow::bail!("please run build command")
    }
    let p = files.first().unwrap();

    Ok(p.clone())
}
