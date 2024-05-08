use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::Command;

fn main() {
    // List of potential credentials (You could also load this from a file)
    let credentials = vec![
        ("miner", "miner"),
        ("root", "root"),
    ];

    // Command to execute on the SSH server
    let command = "echo $HOSTNAME"; // Replace with your actual command

    // Load IP addresses from file
    let ip_addresses = read_ips_from_file("ips.txt").expect("Failed to read IP addresses");

    for ip_address in ip_addresses {
        println!("Attempting to connect to {}", ip_address);
        if let Some(success) = try_credentials(&ip_address, &credentials, command) {
            println!("Success for {}: {}", ip_address, success);
            // break; // Optional: remove if you want to try all IPs even after a success
        } else {
            println!("All credentials failed for {}", ip_address);
        }
    }
}

// Function to read IP addresses from a file
fn read_ips_from_file(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut ips = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() { // Skip empty lines
            ips.push(line);
        }
    }

    Ok(ips)
}

// Function to try a list of credentials
fn try_credentials(ip_address: &str, credentials: &Vec<(&str, &str)>, command: &str) -> Option<String> {
    for (username, password) in credentials {
        println!("Trying {}@{}", username, ip_address);
        match execute_ssh_command(ip_address, username, password, command) {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in stdout");
                    return Some(stdout);
                } else {
                    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8 in stderr");
                    println!("Error for {}@{}: {}", username, ip_address, stderr);
                }
            }
            Err(e) => println!("Failed to connect to {}@{}: {}", username, ip_address, e),
        }
    }
    None
}

// Function to execute SSH command using sshpass
fn execute_ssh_command(ip_address: &str, username: &str, password: &str, command: &str) -> Result<std::process::Output, String> {
    Command::new("sshpass")
        .arg("-p")
        .arg(password)
        .arg("ssh")
        .arg("-o")
        .arg("StrictHostKeyChecking=no")
        .arg(format!("{}@{}", username, ip_address))
        .arg(command)
        .output()
        .map_err(|e| e.to_string())
}
