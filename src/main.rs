mod services;
mod units;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// how long to wait for startup (sec)
    wait_sec: String,
    /// The path to the unit files
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let services = units::parse_dir(&args.path);

    println!("{} services were found", services.len());
    for service in &services {
        println!("{}", service.unit.name);
    }

    services::start_services(&services, args.wait_sec.parse().unwrap_or(0));

    println!("all processes finished");
}
