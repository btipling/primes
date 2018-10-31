extern crate clap;

use std::time::{
    Duration,
    SystemTime,
};
use clap::{Arg, App};

fn main() {
    println!("Starting up!");

    let matches = App::new("Primes")
        .version("1.0")
        .author("Bjorn T. <bjorn@ambientchill.com>")
        .about("Finds primes")
        .arg(Arg::with_name("method")
            .short("m")
            .long("method")
            .value_name("METHOD")
            .help("The method to find primes with. Options are linear. Default is linear.")
            .takes_value(true))
        .get_matches();

    let method = matches.value_of("method").unwrap_or("linear");

    println!("{:?}", method);
    let mut done = false;
    let mut duration = Duration::new(0, 0);
    let start = SystemTime::now();
    let wait_time = 1; // In seconds.
    let mut primes = Vec::new();
    let mut index = 0;
    while !done {
        let now = SystemTime::now();
        index += 1;
        match now.duration_since(start) {
            Err(e) => {
                println!("error getting time: {:?}", e);
                done = true;
            }
            Ok(v) => {
                duration = v;
                if v.as_secs() >= wait_time {
                    primes.push(index);
                    done = true;
                }
            }
        }
    }
    println!("Found primes: {:?}", primes);
    println!("All done! Program ran for {:?} seconds", duration.as_secs());
}
