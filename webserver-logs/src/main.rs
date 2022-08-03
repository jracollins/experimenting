use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::net::Ipv4Addr;
use std::path::Path;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let mut ip_count_map = HashMap::<Ipv4Addr, usize>::new();

    let file_path = Path::new(&args.file);

    let file = File::open(file_path).expect("File Does not exist!");

    let lines = io::BufReader::new(file).lines();

    let re = Regex::new(r"(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)").unwrap();

    for log_line in lines.flatten() {
        for capture in re.captures_iter(&log_line) {
            // index 0 will be the source IP address and only one we are interested in
            if let Ok(ipv4) = Ipv4Addr::from_str(&capture[0]) {
                ip_count_map
                    .entry(ipv4)
                    .and_modify(|current_count| {
                        *current_count += 1;
                    })
                    .or_insert(1);
            }
        }
    }

    let mut ip_vec = Vec::from_iter(ip_count_map.into_iter());

    ip_vec.sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));

    for (ip, count) in ip_vec {
        println!("{} {}", count, ip);
    }
}
