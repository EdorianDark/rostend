mod units;
use std::env;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// how long to wait for startup
    wait_ms: String,
    /// The path to the unit files
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let services = units::parse_dir(&args.path);
    dbg!(&services.len());
    for service in services {
        println!("{}", service.unit.name);
    }

    //start services

    //monitor

    //shut down

    println!("Hello, world!");
}
