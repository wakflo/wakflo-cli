use std::{fs};
use std::io::Write;
use crate::utils::types::WakfloConfig;

pub(crate) fn setup_wakflo_dir() -> anyhow::Result<()> {
    let home_dir = dirs::home_dir().expect("can't read home path");
    let mut wakflo_dir = home_dir.clone();
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

pub(crate) fn get_wakflo_config() -> anyhow::Result<WakfloConfig> {
    let home_dir = dirs::home_dir().expect("can't read home path");
    let mut wakflo_config_dir = home_dir.clone();
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
    let mut wakflo_config_dir = home_dir.clone();
    wakflo_config_dir.extend(&[".wakflo/config.json"]);
    let value = serde_json::to_value(config)?;
    let json = serde_json::to_string_pretty(&value)?;
    let mut file = fs::File::create(wakflo_config_dir)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}