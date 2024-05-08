use std::fs::{self, OpenOptions, File};
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;

fn main() {
    // Read IP addresses from file
    let ip_addresses = read_ips_from_file("ips.txt").expect("Failed to read IP addresses");

    // Error messages to search for in the logs
    let fail_substrings = vec![
        "chain avg vol drop",
        "!!! reg crc error",
        "fail to read pic",
    ];

    for ip_address in ip_addresses {
        println!("Connecting to: {}", ip_address);
        fetch_and_check_logs(&ip_address, &fail_substrings);
    }
}

fn fetch_and_check_logs(ip_address: &str, fail_substrings: &[&str]) {
    let username = "miner";
    let command = "cat /var/log/miner.log"; // Command to fetch logs

    // Execute SSH command
    let output = Command::new("ssh")
        .arg(format!("{}@{}", username, ip_address))
        .arg(command)
        .output()
        .expect("Failed to execute SSH command");

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in stdout");
        println!("Logs fetched for {}: ", ip_address);
        fs::write("miner_log.txt", &stdout).expect("Unable to write file");

        // Check logs for errors
        println!("Searching for errors in logs of {}:\n", ip_address);
        check_for_errors(&stdout, fail_substrings);
    } else {
        let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8 in stderr");
        println!("SSH error for {}: {}", ip_address, stderr);
    }

    // Optionally clear the log file
    clear_log_file();
}

fn check_for_errors(logs: &str, fail_substrings: &[&str]) {
    let mut found_errors = false;
    for error_msg in fail_substrings {
        if logs.contains(error_msg) {
            println!("Error found: {}", error_msg);
            found_errors = true;
        }
    }
    if !found_errors {
        println!("No errors found.");
    }
}

fn clear_log_file() {
    let file = OpenOptions::new()
        .write(true)
        .open("miner_log.txt")
        .expect("Unable to open file");
    file.set_len(0).expect("Failed to clear log file");
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
