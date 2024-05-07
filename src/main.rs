use std::process::Command;
use std::io::{self, Write};
use std::fs;

fn main() {
    // SSH connection details
    let username = "miner";
    let mut hostname = String::new();
    let command_to_execute = "tail -n 100 /var/log/messages.0"; // Command to execute on the SSH server

    //read the host name
    println!("Enter the miner's IP address: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut hostname).expect("Failed to read line");

    hostname = hostname.trim().to_string();

    // SSH command
    let output = Command::new("ssh")
        .arg(format!("{}@{}", username, hostname))
        .arg(command_to_execute)
        .output()
        .expect("Failed to execute SSH command");

    // Check if the command executed successfully
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in stdout");
        println!("Output:\n{}", stdout);
        fs::write("C:\\Users\\Mason\\Desktop\\code shit\\Rust\\miner_scan\\miner_log.txt", stdout).expect("Unable to write file");
    } else {
        let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8 in stderr");
        println!("Error:\n{}", stderr);
    }
}