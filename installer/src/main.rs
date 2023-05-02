use owo_colors::OwoColorize;
use std::process::Command;

pub fn install_sccache() {
    println!("{}", "> cargo install sccache".bright_cyan());

    // Install sccache
    let status = Command::new("cargo")
        .arg("install")
        .arg("sccache")
        .arg("-f")
        .status()
        .unwrap();

    if status.success() {
        println!("{}", "Installed SCCache".bright_green());
    } else {
        println!("{}", "Failed to install SCCache".bright_red());
    }
}

pub fn set_compiler_nightly() {
    println!("{}", "> rustup default nightly".bright_cyan());

    // Set nightly compiler
    let status = Command::new("rustup")
        .arg("default")
        .arg("nightly")
        .status()
        .unwrap();

    if status.success() {
        println!(
            "{} {} {}",
            "Switched to".bright_green(),
            "nightly".bright_blue(),
            "compiler".bright_green(),
        );
    } else {
        println!("{}", "Failed to set nightly compiler".bright_red());
    }
}

pub fn install_wakflo() {
    println!("{}", "> cargo install wakflo-rs".bright_cyan());

    // Install Wakflo CLI
    let status = Command::new("cargo")
        .arg("install")
        .arg("wakflo-rs")
        .arg("-f")
        .status()
        .unwrap();

    if status.success() {
        println!("{}", "Installed Wakflo CLI".bright_green());
    } else {
        println!("{}", "Failed to install Wakflo CLI".bright_red());
    }
}

fn main() {
    println!("🚀 Installing Wakflo CLI");

    match std::env::consts::OS {
        "macos" => {
            println!("💻 Installing for MacOS");

            println!(
                "{}",
                format!(
                    "{} {} {} {} ",
                    "Please ensure you have the following packages installed:",
                    "zld\n".bright_cyan(),
                    "brew:",
                    "brew install michaeleisel/zld/zld".bright_yellow()
                )
            );

            set_compiler_nightly();
            install_sccache();
            install_wakflo();

            println!("{}", "Installation complete".bright_green());
        }
        "linux" => {
            println!("💻 Installing for Linux");

            println!(
                "{}",
                format!(
                    "{} {} {} {} {} {}",
                    "Please ensure you have the following packages installed:",
                    "lld, clang\n".bright_cyan(),
                    "Pacman:",
                    "sudo pacman -S clang lld\n".bright_yellow(),
                    "Apt:",
                    "sudo apt install clang lld".bright_yellow()
                )
            );

            set_compiler_nightly();
            install_sccache();
            install_wakflo();

            println!("{}", "Installation complete".bright_green());
        }
        "windows" => {
            println!("💻 Installing for Windows");

            let _ = enable_ansi_support::enable_ansi_support();

            set_compiler_nightly();
            install_sccache();
            install_wakflo();

            println!("{}", "Installation complete".bright_green());
        }
        _ => {
            println!("OS Not Supported");
        }
    }
}