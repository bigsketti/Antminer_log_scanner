use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::Command;

// Function to read IP addresses from a file
pub fn try_credentials(ip_address: &str, credentials: &Vec<(&str, &str)>, fail_substrings: &Vec<&str>) {
    for (username, password) in credentials {
        println!("Trying {}@{}", username, ip_address);

        let output = run_command(ip_address, username, password);

        handle_output(output, ip_address, username, fail_substrings);
    }
}

//iterates through the logs to find errors
fn log_contains_errors(logs: &str, fail_substrings: &Vec<&str>) -> bool {
    fail_substrings.iter().any(|&error| logs.contains(error))
}

//finds ip addresses from ips.txt
pub fn read_ips_from_file(filename: &str) -> io::Result<Vec<String>> {
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

//runs the ssh command
fn run_command(ip_address: &str, username: &str, password: &str) -> Option<Result<std::process::Output, std::io::Error>> {
    //log files to check for logs because bitmain makes it difficult
    let log_files_to_check: Vec<&str> = vec![
            "cat /var/log/miner.log",
            "cat /var/log/messages.0",
            "cat /var/log/messages",
            "cat /var/log/dmesg",
        ];

    //sends the ssh command to the miner    
    for log_file in &log_files_to_check {
        let command = log_file; // Command to fetch logs
        let output = Command::new("sshpass")
        .arg("-p")
        .arg(password)
        .arg("ssh")
        .arg("-o").arg("StrictHostKeyChecking=no")
        .arg("-o").arg("UserKnownHostsFile=/dev/null")
        .arg(format!("{}@{}", username, ip_address))
        .arg(command)
        .output();

        return Some(output);
    }
    None
}

//handles the output of the ssh command
fn handle_output(output: Option<Result<std::process::Output, std::io::Error>>, ip_address: &str, username: &str, fail_substrings: &Vec<&str>) {
    match output {
        Some(Ok(output)) if output.status.success() => {
            let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in stdout");

            // Save logs to a file named after the IP address
            let file_name = format!("{}_log.txt", ip_address.replace(".", "_")); // replace dots with underscores for filename
            std::fs::write(&file_name, &stdout).expect("Unable to write file");
            println!("Logs saved to {}", file_name);

            if log_contains_errors(&stdout, fail_substrings) {
                println!("Errors found in logs for {}", ip_address);
                println!("{}", fail_substrings.join("\n"));
            } else {
                println!("No errors found in logs for {}", ip_address);
            }
        },

        Some(Ok(output)) => {
            let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8 in stderr");
            println!("SSH error for {}@{}: {}", username, ip_address, stderr);
        },

        Some(Err(e)) => println!("Failed to connect to {}@{}: {}", username, ip_address, e),
        None => todo!(),
    }
}

// Path: src/iterate.rs