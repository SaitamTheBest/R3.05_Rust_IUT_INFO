use port_scanner::is_open_sync;
use port_scanner::Options;
use clap::Parser;
use std::time::Instant;
fn main() {
    let instant = Instant::now();
    let options = Options::parse();
    let host = options.host;
    let port_min = options.port_min;
    let port_max = options.port_max;
    let timeout = options.timeout;
    for port in port_min..port_max {
        if is_open_sync(&host, port, timeout) {
            println!("{} is open", port);
        }
    }

    println!("Finished in {:?} seconds", instant.elapsed());
}