use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::convert::AsRef;
use std::io::Write;
use std::{fs, path};
use strum_macros::AsRefStr;
use strum_macros::{EnumIter, EnumString};
use wakflo_sdk::Wakflo;

#[derive(Debug, Serialize, Deserialize, EnumString, EnumIter, AsRefStr)]
pub(crate) enum Lang {
    Rust,
    Typescript,
    Javascript,
    Golang,
    Python,
    Php,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TaskConfig {
    pub name: String,
    pub description: String,
    pub lang: Lang,
    pub category: String,
}

pub(crate) fn resolve_variables(code: &str, config: &TaskConfig) -> anyhow::Result<String> {
    let name = config.name.to_case(Case::Kebab).to_lowercase();
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .expect("failed to parse template")
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
    "title": "Input",
    "type": "object",
    "properties": {

    }
  },
  "output": {
    "title": "Output",
    "type": "object",
    "properties": {

    }
  }
}
"#;

/// generate wakflo plugin config
pub(crate) fn generate_shared_plugin_files(config: &TaskConfig) -> anyhow::Result<()> {
    let dir_name = config.name.clone().to_case(Case::Kebab).to_lowercase();

    let mut wak = Wakflo::default();
    wak.plugin.name = dir_name.clone();
    wak.plugin.description = config.description.to_owned();
    wak.plugin.category = config.category.to_owned();
    wak.plugin.icon = "streamline:programming-module-cube-code-module-programming-plugin".to_owned();
    wak.plugin.documentation = Some("".to_owned());

    wak.dependencies = Some(vec![]);

    let mut resolved_str = resolve_variables(WAKFLO_TOML, config)?;
    let mut file = fs::File::create(path::Path::new(
        format!("{}/wakflo.toml", dir_name).as_str(),
    ))?;
    file.write_all(resolved_str.as_bytes())?;

    resolved_str = resolve_variables(README, config)?;
    file = fs::File::create(path::Path::new(format!("{}/README.md", dir_name).as_str()))?;
    file.write_all(resolved_str.as_bytes())?;

    resolved_str = resolve_variables(PROPERTIES, config)?;
    file = fs::File::create(path::Path::new(
        format!("{}/properties.json", dir_name).as_str(),
    ))?;
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
