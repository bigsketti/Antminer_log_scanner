use std::process::Command;
use std::io::{self, Write};
use std::fs::File;
use std::fs;

fn main() {
    // SSH connection details
    let username = "miner";
    let mut hostname = String::new();
//    let command = "tail -n 1000 /var/log/messages.0"; // Command to execute on the SSH server
    let command = "cat /var/log/miner.log"; // Command to execute on the SSH server

    //read the host name
    println!("Enter the miner's IP address or q to quit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut hostname).expect("Failed to read line");

    hostname = hostname.trim().to_string();

    if hostname.is_empty() {
        println!("No hostname provided");
    } else if hostname == "q" {
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
        println!("Logs:\n{}", stdout);
        fs::write("miner_log.txt", stdout).expect("Unable to write file");
        println!("logs fetched and written, searching for errors\n")
    } else {
        let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8 in stderr");
        println!("Error:\n{}", stderr);
    }

    let fail_substrings: Vec<String> = vec![
                                            String::from("chain avg vol drop"),
                                            String::from("!!! reg crc error"),
                                            String::from("fail to read pic")
                                            ]; // to be filled later with antminer error log messages

    let log = fs::read_to_string("miner_log.txt").expect("Unable to read file");

    if log.is_empty() {
        println!("no logs found\n")
    } else {
        println!("Errors found in logs:\n");
        for i in fail_substrings {
            if log.contains(&i) {
                println!("{}", i);
                return;
            }
        }
    }
    
    File::create("miner_log.txt").expect("Unable to open file");

}
