use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::process::Command;

fn main() {
    // Credentials defined in the code for cycling through
    let credentials = vec![
        ("miner", "miner"),
        ("root", "root"),
    ];

    // Error messages to search for in the logs
    let fail_substrings = vec![
        "chain avg vol drop",
        "!!! reg crc error",
        "fail to read pic",
    ];

    // Read IP addresses from file
    let ip_addresses = read_ips_from_file("ips.txt").expect("Failed to read IP addresses");

    for ip_address in ip_addresses {
        println!("Attempting to connect to {}", ip_address);
        if let Some(success) = try_credentials(&ip_address, &credentials, &fail_substrings) {
            println!("Success for {}: {}", ip_address, success);
        } else {
            println!("All credentials failed for {}", ip_address);
        }
    }
}

fn try_credentials(ip_address: &str, credentials: &Vec<(&str, &str)>, fail_substrings: &Vec<&str>) -> Option<String> {
    for (username, password) in credentials {
        println!("Trying {}@{}", username, ip_address);
        let command = "cat /var/log/miner.log"; // Command to fetch logs
        let output = Command::new("sshpass")
            .arg("-p")
            .arg(password)
            .arg("ssh")
            .arg("-o").arg("StrictHostKeyChecking=no")
            .arg("-o").arg("UserKnownHostsFile=/dev/null")
            .arg(format!("{}@{}", username, ip_address))
            .arg(command)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in stdout");
                // Now check for errors in logs
                if log_contains_errors(&stdout, fail_substrings) {
                    return Some(format!("Errors found in logs for {}: {}", ip_address, stdout));
                }
                return Some(format!("No errors in logs for {}", ip_address));
            },
            Ok(output) => {
                let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8 in stderr");
                println!("SSH error for {}@{}: {}", username, ip_address, stderr);
            },
            Err(e) => println!("Failed to connect to {}@{}: {}", username, ip_address, e),
        }
    }
    None
}

fn log_contains_errors(logs: &str, fail_substrings: &Vec<&str>) -> bool {
    fail_substrings.iter().any(|&error| logs.contains(error))
}

fn read_ips_from_file(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut ips = Vec::new();
    for line in reader.lines() {
    let line = line?;
    if !line.trim().is_empty() {
        ips.push(line);
    }
}
Ok(ips)
}
