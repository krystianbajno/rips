use ipnetwork::IpNetwork;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: rips <subnet>");
        process::exit(1);
    }

    let cidr = &args[1];
    match cidr.parse::<IpNetwork>() {
        Ok(network) => {
            for ip in network.iter() {
                println!("{}", ip);
            }
        }
        Err(_) => {
            eprintln!("Invalid CIDR format: {}", cidr);
            process::exit(1);
        }
    }
}
