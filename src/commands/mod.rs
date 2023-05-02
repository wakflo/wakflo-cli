mod auth;
mod task;

use crate::utils::dir_files::setup_wakflo_dir;
use auth::AuthCommand;
use clap::{CommandFactory, Parser, Subcommand};
use task::TaskCommand;

// #[derive(Debug, Args)]
// struct GlobalOpts {
//     /// Color
//     #[clap(long, global = true, default_value_t = Color::Auto)]
//     color: Color,
//
//     /// Verbosity level (can be specified multiple times)
//     #[clap(long, short, global = true, parse(from_occurrences))]
//     verbose: usize,
// }

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

    /// Tasks commands
    Task {
        #[clap(subcommand)]
        plugin: TaskCommand,
    },
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
            Commands::Task { plugin } => match plugin {
                TaskCommand::New { name } => TaskCommand::new_plugin(name),
                TaskCommand::Publish => Ok(()),
                TaskCommand::Test => Ok(()),
                TaskCommand::Run => TaskCommand::run_plugin(),
            },
        };

        if let Err(e) = rsp {
            // loading.fail(format!("{}", e))
            println!("{}", e)
        }

        // loading.end();

        Ok(())
    }
}
