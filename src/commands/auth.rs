<<<<<<< HEAD
use crate::api::make_api;
use crate::utils::dir_files::{get_wakflo_config, update_wakflo_config};
use clap::Subcommand;
use console::Style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use loading::Loading;
=======
use clap::{Subcommand};
use console::Style;
use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;
use loading::Loading;
use crate::api::make_api;
use crate::utils::dir_files::{get_wakflo_config, update_wakflo_config};
>>>>>>> 85173fa (feat: first commit)

#[derive(Subcommand)]
pub enum AuthCommand {
    /// Login as a user
    Login {
        /// user email or username
        #[arg(required = false, short, long)]
        identity: Option<String>,

        /// user password
        #[arg(required = false, short, long)]
        password: Option<String>,
    },
    /// Get current user
    Whoami,
    /// Logouts out current user
    Logout,
}

impl AuthCommand {
<<<<<<< HEAD
=======

>>>>>>> 85173fa (feat: first commit)
    pub fn login(identity: Option<String>, password: Option<String>) -> anyhow::Result<()> {
        let theme = ColorfulTheme {
            values_style: Style::new().yellow().dim(),
            ..ColorfulTheme::default()
        };
        println!("Login to wakflo.ai");

        #[allow(unused_assignments)]
        let mut email: String = "".to_owned();
<<<<<<< HEAD
        if identity.is_none() {
            email = Input::with_theme(&theme)
                .with_prompt("Identity (Email)")
                .interact()
                .expect("missing");
        } else {
            email = identity.unwrap_or_default();
=======
        if let None = identity {
            email = Input::with_theme(&theme)
                .with_prompt("Identity (Email)")
                .interact().expect("missing");
        } else {
            email = identity.expect("user email missing");
>>>>>>> 85173fa (feat: first commit)
        }

        #[allow(unused_assignments)]
        let mut pass: String = "".to_owned();
<<<<<<< HEAD
        if password.is_none() {
            pass = Input::with_theme(&theme)
                .with_prompt("Password")
                .allow_empty(true)
                .interact()
                .expect("missing");
        } else {
            pass = password.unwrap_or_default();
=======
        if let None = password {
            pass = Input::with_theme(&theme)
                .with_prompt("Password")
                .allow_empty(true)
                .interact().expect("missing");
        } else {
            pass = password.expect("user password missing");
>>>>>>> 85173fa (feat: first commit)
        }

        let loading = &Loading::default();
        loading.text("Login in ...");

        let auth_user = make_api().auth.login(email, pass)?;
        let mut config = get_wakflo_config()?;
        config.auth = Some(auth_user);
        update_wakflo_config(config)?;

        loading.success("Logged in successfully");
        Ok(())
<<<<<<< HEAD
=======

>>>>>>> 85173fa (feat: first commit)
    }

    pub fn whoami() -> anyhow::Result<()> {
        let loading = &Loading::default();
        loading.text("Fetching user data ...");
        match make_api().auth.whoami() {
<<<<<<< HEAD
            Ok(auth_user) => {
=======
            Ok(auth_user ) => {
>>>>>>> 85173fa (feat: first commit)
                loading.success(format!("Logged in as {}", auth_user.email));
                loading.end();
                Ok(())
            }
            Err(e) => {
<<<<<<< HEAD
                loading.fail(format!("Error: {}", e));
=======
                loading.fail(format!("Error: {}", e.to_string()));
>>>>>>> 85173fa (feat: first commit)
                loading.end();
                Err(e)
            }
        }
    }
<<<<<<< HEAD
}
=======
}
>>>>>>> 85173fa (feat: first commit)
