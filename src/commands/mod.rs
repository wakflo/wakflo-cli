mod auth;
mod new;
mod global;

use crate::utils::dir_files::{setup_wakflo_dir};
use auth::AuthCommand;
use clap::{CommandFactory, Parser, Subcommand};
use crate::commands::global::GlobalCommand;
use crate::commands::new::NewCommand;
use regex::Regex;

/// Wakflo
///
/// A commandline utility for wakflo ai services
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "wakflo")]
#[command(bin_name = "wakflo")]
pub struct WakfloCli {
    // #[clap(flatten)]
    // global_opts: GlobalOpts,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authentication commands
    Auth {
        #[clap(subcommand)]
        auth: AuthCommand,
    },

    /// New plugins commands
    New {
        #[clap(subcommand)]
        plugin: NewCommand,
    },

    /// Tests the current wakflo plugin
    Test,

    /// Builds the current wakflo plugin
    Build,

    /// Run the current wakflo plugin
    Run,

    /// Deploys the current wakflo plugin
    Deploy,

    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}

impl WakfloCli {
    pub fn init() -> anyhow::Result<()> {
        let cli = WakfloCli::parse();
        setup_wakflo_dir().expect("failed to create wakflo config");

        let rsp = match cli.command {
            // e.g. `$ cli completions bash`
            Commands::Completions { shell } => {
                shell.generate(&mut WakfloCli::command(), &mut std::io::stdout());
                Ok(())
            }
            Commands::Auth { auth } => match auth {
                AuthCommand::Login { password, identity } => AuthCommand::login(identity, password),
                AuthCommand::Whoami => AuthCommand::whoami(),
                AuthCommand::Logout => Ok(()),
            },
            Commands::New { plugin } => match plugin {
                NewCommand::Task { name } => NewCommand::new_task(name),
            },
            Commands::Test => GlobalCommand::test_plugin(),
            Commands::Run => GlobalCommand::run_plugin(),
            Commands::Build => GlobalCommand::build_plugin(),
            Commands::Deploy => GlobalCommand::deploy_plugin(),
        };

        if let Err(e) = rsp {
            let raw_msg = e.to_string().clone();
            let re = Regex::new(r"^https?:\\/\\/(?:www\\.)?[-a-zA-Z0-9@:%._\\+~#=]{1,256}\\.[a-zA-Z0-9()]{1,6}\\b(?:[-a-zA-Z0-9()@:%_\\+.~#?&\\/=]*)$").expect("");
            let msg = re.replace_all(raw_msg.as_str(), "");
            println!("oops!: {}", msg.to_string())
        }

        Ok(())
    }
}
