mod auth;
mod plugin;

<<<<<<< HEAD
use crate::utils::dir_files::setup_wakflo_dir;
use auth::AuthCommand;
use clap::{CommandFactory, Parser, Subcommand};
use plugin::PluginCommand;
=======
use clap::{CommandFactory, Parser, Subcommand};
use auth::AuthCommand;
use plugin::PluginCommand;
use crate::utils::dir_files::setup_wakflo_dir;

>>>>>>> 85173fa (feat: first commit)

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
<<<<<<< HEAD
=======

>>>>>>> 85173fa (feat: first commit)
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authentication commands
    Auth {
        #[clap(subcommand)]
<<<<<<< HEAD
        auth: AuthCommand,
    },

    /// Plugins commands
    Plugin {
        #[clap(subcommand)]
        plugin: PluginCommand,
=======
        auth: AuthCommand
    },

    /// Plugins commands
    Plugin{
        #[clap(subcommand)]
        plugin: PluginCommand
>>>>>>> 85173fa (feat: first commit)
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
<<<<<<< HEAD
            Commands::Auth { auth } => match auth {
                AuthCommand::Login { password, identity } => AuthCommand::login(identity, password),
                AuthCommand::Whoami => AuthCommand::whoami(),
                AuthCommand::Logout => Ok(()),
            },
            Commands::Plugin { plugin } => match plugin {
                PluginCommand::New { name } => PluginCommand::new_plugin(name),
                PluginCommand::Publish => Ok(()),
                PluginCommand::Test => Ok(()),
                PluginCommand::Run => PluginCommand::run_plugin(),
            },
        };

=======
            Commands::Auth { auth } => {
                match auth {
                    AuthCommand::Login { password, identity } =>
                        AuthCommand::login(identity, password),
                    AuthCommand::Whoami =>
                        AuthCommand::whoami(),
                    AuthCommand::Logout => Ok(())
                }
            }
            Commands::Plugin { plugin } => {
                match plugin {
                    PluginCommand::New { name } => PluginCommand::new_plugin(name),
                    PluginCommand::Publish => Ok(()),
                    PluginCommand::Test => Ok(()),
                    PluginCommand::Run => PluginCommand::run_plugin(),
                }
            }
        };


>>>>>>> 85173fa (feat: first commit)
        if let Err(e) = rsp {
            // loading.fail(format!("{}", e))
            println!("{}", e)
        }

        // loading.end();

        Ok(())
    }
<<<<<<< HEAD
}
=======
}
>>>>>>> 85173fa (feat: first commit)
