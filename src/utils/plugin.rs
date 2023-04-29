use std::{fs, path};
use std::io::Write;
use convert_case::{Case, Casing};
use serde::{Serialize, Deserialize};
use strum_macros::{EnumString, EnumIter};
use std::convert::AsRef;
use strum_macros::AsRefStr;

#[derive(Debug, Serialize, Deserialize, EnumString, EnumIter, AsRefStr)]
pub(crate) enum Lang {
    Rust,
    Typescript,
    Javascript,
    Golang,
    Python,
    Php
}


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PluginConfig {
    pub name: String,
    pub description: String,
    pub lang: Lang,
    pub category: String,
}

pub(crate) fn resolve_variables(code: &str, config: &PluginConfig) -> anyhow::Result<String> {
    let name = config.name.clone().to_case(Case::Kebab).to_lowercase();
    let template = liquid::ParserBuilder::with_stdlib()
        .build().unwrap()
        .parse(code)?;

    let lang = config.lang.as_ref();

    let globals = liquid::object!({
        "name": name,
        "description": config.description,
        "displayName": config.name,
        "category": config.category,
        "lang": lang,
    });

    Ok(template.render(&globals)?)
}

pub(crate) const WAKFLO_TOML: &str = r#"[plugin]
name = "{{ name }}"
version = "0.0.1"
description = "{{ description }}"
language = "{{ lang }}"
category = "{{ category }}"

[dependencies]
"#;

pub(crate) const README: &str = r#"## {{ name }}
Getting Started
"#;



pub(crate) const PROPERTIES: &str = r#"{
  "input": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "title": "Input",
    "type": "object",
    "properties": {

    }
  },
  "output": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "title": "Output",
    "type": "object",
    "properties": {

    }
  }
}
"#;


/// generate wakflo plugin config
pub(crate) fn generate_shared_plugin_files(config: &PluginConfig) -> anyhow::Result<()> {
    let dir_name = config.name.clone().to_case(Case::Kebab).to_lowercase();

    let mut resolved_str = resolve_variables(WAKFLO_TOML, &config)?;
    let mut file = fs::File::create(path::Path::new(format!("{}/wakflo.toml", dir_name).as_str()))?;
    file.write_all(resolved_str.as_bytes())?;

    resolved_str = resolve_variables(README, &config)?;
    file = fs::File::create(path::Path::new(format!("{}/README.md", dir_name).as_str()))?;
    file.write_all(resolved_str.as_bytes())?;

    resolved_str = resolve_variables(PROPERTIES, &config)?;
    file = fs::File::create(path::Path::new(format!("{}/properties.json", dir_name).as_str()))?;
    file.write_all(resolved_str.as_bytes())?;

    Ok(())
}

pub(crate) fn string_to_lang(lang: &str) -> Lang {
    if lang == "Rust" {
        Lang::Rust
    } else if lang == "Golang" {
        Lang::Golang
    } else if lang == "Typescript" {
        Lang::Typescript
    } else if lang == "Javascript" {
        Lang::Javascript
    } else if lang == "PHP" {
        Lang::Php
    } else {
        Lang::Rust
    }
}
