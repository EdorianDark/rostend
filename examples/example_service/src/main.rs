use std::env;
use std::thread;
use std::time;

fn main() {
	let args: Vec<String> = env::args().collect();
	let wait : u64 = args.get(1).unwrap_or(&"2".to_string()).parse().unwrap_or(2);
	
    println!("example_service: start waiting {} seconds", wait);
	
	let seconds = time::Duration::from_secs(wait);
    thread::sleep(seconds);

    println!("example_service: end waiting");
}
