mod iterate;

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
        "fail to write 1",
        "fail to read 0:1",
    ];

    // Read IP addresses from file
    let ip_addresses = crate::iterate::read_ips_from_file("ips.txt").expect("Failed to read IP addresses");

    for ip_address in ip_addresses {
        println!("Attempting to connect to {}", ip_address);
        crate::iterate::try_credentials(&ip_address, &credentials, &fail_substrings);
    }
}

// Path: src/main.rs