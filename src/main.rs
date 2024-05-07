use std::process::Command;
use std::io::{self, Write};
use std::fs;

fn main() {
    // SSH connection details
    let username = "miner";
    let mut hostname = String::new();
    let command = "tail -n 1000 /var/log/messages.0"; // Command to execute on the SSH server

    //read the host name
    println!("Enter the miner's IP address: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut hostname).expect("Failed to read line");

    hostname = hostname.trim().to_string();

    if hostname.is_empty() {
        println!("No hostname provided");
        return;
    }

    // SSH command
    let output = Command::new("ssh")
        .arg(format!("{}@{}", username, hostname))
        .arg(command)
        .output()
        .expect("Failed to execute SSH command");

    // Check if the command executed successfully
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in stdout");
        fs::write("\\miner_scan\\miner_log.txt", stdout).expect("Unable to write file");
        println!("logs fetched and written, searching for errors\n")
    } else {
        let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8 in stderr");
        println!("Error:\n{}", stderr);
    }

    const FAIL_SUBSTRINGS: Vec<String> = vec![]; // to be filled later with antminer error log messages

    let log = fs::read_to_string("\\miner_scan\\miner_log.txt").expect("Unable to read file");

    for i in FAIL_SUBSTRINGS {
        if log.contains(&i) {
            println!("{}", i);
            return;
        }
    }

    use std::fs::File;
    
    File::create("/path/to/file.txt").expect("Unable to open file");

}