use crate::api::make_api;
use crate::templates::rust::create_rust_plugin_project;
use crate::utils::plugin::{string_to_lang, Lang, TaskConfig};
use clap::Subcommand;
use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use loading::Loading;
use crate::templates::go::create_go_plugin_project;

#[derive(Subcommand)]
pub enum NewCommand {
    /// generates new wakflo task plugin
    #[command(arg_required_else_help = false)]
    Task {
        /// name of the plugin
        name: Option<String>,
    },
}

impl NewCommand {
    pub fn new_task(name: Option<String>) -> anyhow::Result<()> {
        let theme = ColorfulTheme {
            values_style: Style::new().yellow().dim(),
            ..ColorfulTheme::default()
        };

        let lang_list = &["Rust", "Typescript", "Javascript", "Golang", "PHP"];

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
        if name.is_none() {
            plugin_name = Input::with_theme(&theme)
                .with_prompt("Name (plugin name)")
                .interact_text()
                .expect("missing");
        } else {
            plugin_name = name.unwrap_or_default();
        }

        let description: String = Input::with_theme(&theme)
            .with_prompt("Description")
            .allow_empty(true)
            .interact()
            .expect("missing description");

        let category_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a category")
            .default(0)
            .items(&category_ids.clone())
            .interact()
            .expect("missing category");

        let lang_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a language")
            .default(0)
            .items(&lang_list[..])
            .interact()
            .expect("missing lang");

        let lang = lang_list[lang_idx];
        let cat = category_ids.get(category_idx).expect("missing category");

        let loading = &Loading::default();
        loading.text(format!("Login creating plugin in {}", lang));
        let config = TaskConfig {
            description,
            name: plugin_name,
            lang: string_to_lang(lang),
            category: cat.clone(),
        };

        NewCommand::create_plugin(config, loading)
    }

    fn create_plugin(config: TaskConfig, loading: &Loading) -> anyhow::Result<()> {
        match config.lang {
            Lang::Rust => create_rust_plugin_project(config, loading),
            Lang::Typescript => Ok(()),
            Lang::Javascript => Ok(()),
            Lang::Golang => create_go_plugin_project(config, loading),
            Lang::Python => Ok(()),
            Lang::Php => Ok(()),
        }
    }
}
