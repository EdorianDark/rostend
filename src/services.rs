// use cgroups_fs; TODO
use crate::units::Service;
use std::process::Command;
use std::thread;
use std::time;
use std::time::Duration;

pub fn start_services(services: &Vec<Service>, wait : u64) {
    let mut children = Vec::new();
    for service in services {
        if service.exec_start.is_some() {
            let mut program = service.exec_start.as_ref().unwrap().split_whitespace();
            let executable = program.next().unwrap();
            let child = Command::new(executable)
                .args(program)
                .spawn()
                .expect("failed to wait on child");
            children.push(child);

            let seconds = time::Duration::from_secs(wait);
            println!("rostend: sleeping for {}", &wait);
            thread::sleep(seconds);
        }
    }

    let mut finished = false;
    while !finished {
        finished = true;
        for child in &mut children {
            let status = child.try_wait();
            if status.is_ok() {
                if status.unwrap().is_none() {
                    finished = false;
                }
            }
        }
        thread::sleep(Duration::from_millis(500));
    }
}
