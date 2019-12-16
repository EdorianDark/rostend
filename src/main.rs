mod units;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let services = units::parse_dir("examples");
    dbg!(&services.len());

    //start services

    //monitor

    //shut down

    println!("Hello, world!");
}
