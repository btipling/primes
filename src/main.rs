use std::time::{
    Duration,
    SystemTime,
};

fn main() {
    println!("Starting up!");
    let mut done = false;
    let mut duration = Duration::new(0, 0);
    let start = SystemTime::now();
    let wait_time = 1; // In seconds.
    while !done {
        let now = SystemTime::now();
        match now.duration_since(start) {
            Err(e) => {
                println!("error getting time: {:?}", e);
                done = true;
            }
            Ok(v) => {
                duration = v;
                if v.as_secs() >= wait_time {
                    done = true;
                }
            }
        }
    }

    println!("All done! Program ran for {:?} seconds", duration.as_secs());
}
