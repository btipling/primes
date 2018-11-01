extern crate clap;
mod method;

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

    let method_selection = matches.value_of("method").unwrap_or("linear");

    let mut done = false;
    let mut duration = Duration::new(0, 0);
    let start = SystemTime::now();
    let wait_time = 1; // In seconds.
    let mut primes = Vec::new();
    let mut index = 0;
    let method: &method::method::Method;
    match method_selection {
        "linear" => {
            method = method::linear::Linear::new();
        },
        _ => panic!("Invalid method option."),
    }

    println!("Using {:?}", method.name());
    while !done {
        let now = SystemTime::now();
        index += 1;
        if method.is_prime(index) {
            primes.push(index);
        }
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
    println!("Found {:?} primes.", primes.len());
    println!("All done! Program ran for {:?} seconds", duration.as_secs());
}
