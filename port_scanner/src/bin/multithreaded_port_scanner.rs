use clap::Parser;
use std::time::Instant;
use port_scanner::Options;
use port_scanner::is_open_sync;

fn main() {
    let instant = Instant::now();
    let options = Options::parse();
    let mut handles = vec![];
    for port in options.port_min..options.port_max {
        let host = options.host.clone();
        let thread = std::thread::spawn(move || {
            if is_open_sync(&host, port, options.timeout) {
                println!("Port {} is open", port);
            } else {
                println!("Port {} is closed", port);
            }
        });

        handles.push(thread);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Time elapsed: {:?}", instant.elapsed());
}