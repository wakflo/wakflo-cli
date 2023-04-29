use clap::{Subcommand};
use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use loading::Loading;
use crate::api::make_api;
use crate::templates::rust::create_rust_plugin_project;
use crate::utils::plugin::{Lang, PluginConfig, string_to_lang};

#[derive(Subcommand)]
pub enum PluginCommand {
    /// generates new wakflo plugin
    #[command(arg_required_else_help = false)]
    New {
        /// name of the plugin
        name: Option<String>,
    },
    Publish,
    Test,
    Run,
}

impl PluginCommand {
    pub fn run_plugin() -> anyhow::Result<()> {
        Ok(())
    }

    pub fn new_plugin(name: Option<String>) -> anyhow::Result<()> {
        let theme = ColorfulTheme {
            values_style: Style::new().yellow().dim(),
            ..ColorfulTheme::default()
        };

        let lang_list = &[
            "Rust",
            "Typescript",
            "Javascript",
            "Golang",
            "PHP",
        ];

        println!("Welcome to Wakflo.AI new plugin setup wizard");

        let categories = make_api().category.list()?;
        let mut category_ids: Vec<String> = vec![];

        for category in categories {
            category_ids.push(category.id);
        }

        if !Confirm::with_theme(&theme)
            .with_prompt("Do you want to continue?")
            .interact()?
        {
            return Ok(());
        }

        #[allow(unused_assignments)]
        let mut plugin_name = String::new();
        if let None = name {
            plugin_name = Input::with_theme(&theme)
                .with_prompt("Name (plugin name)")
                .interact_text().expect("missing");
        } else {
            plugin_name = name.expect("plugin name missing");
        }

        let description: String = Input::with_theme(&theme)
            .with_prompt("Description")
            .allow_empty(true)
            .interact().expect("missing");

        let category_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a category")
            .default(0)
            .items(&category_ids.clone())
            .interact()
            .unwrap();

        let lang_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a language")
            .default(0)
            .items(&lang_list[..])
            .interact()
            .unwrap();

        let lang = lang_list[lang_idx];
        let cat = category_ids.get(category_idx).expect("missing category");

        let loading = &Loading::default();
        loading.text(format!("Login creating plugin in {}", lang));
        let config = PluginConfig {
            description,
            name: plugin_name,
            lang: string_to_lang(lang),
            category: cat.clone(),
        };

        PluginCommand::create_plugin(config, loading)
    }

    fn create_plugin(config: PluginConfig, loading: &Loading) -> anyhow::Result<()> {
        match config.lang {
            Lang::Rust => create_rust_plugin_project(config, loading),
            Lang::Typescript => Ok(()),
            Lang::Javascript => Ok(()),
            Lang::Golang => Ok(()),
            Lang::Python => Ok(()),
            Lang::Php => Ok(()),
        }
    }
}