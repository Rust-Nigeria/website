use std::process::{Command, exit};

fn run_command(command: &str, args: &[&str]) {
    println!("Running: {} {}", command, args.join(" "));
    let status = Command::new(command)
        .args(args)
        .status()
        .expect("Failed to execute process");

    if !status.success() {
        eprintln!("Command failed with exit code: {:?}", status.code());
        exit(status.code().unwrap_or(1));
    }
}

fn main() {
    // Sequentially run cargo commands
    run_command("cargo", &["build"]);
    run_command("cargo", &["test"]);
    run_command("cargo", &["clippy"]);
    run_command("cargo", &["fmt"]);

    println!("All commands executed successfully!");
}
