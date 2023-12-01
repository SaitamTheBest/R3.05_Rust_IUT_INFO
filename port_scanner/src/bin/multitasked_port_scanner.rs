use std::time::Instant;
use port_scanner::{is_open_async, Options};
use clap::Parser;

#[tokio::main]
async fn main() {
    let instant = Instant::now();
    let options = Options::parse();
    let host = options.host;
    let port_min = options.port_min;
    let port_max = options.port_max;
    let timeout = options.timeout;
    for port in port_min..port_max {
        if is_open_async(&host, port, timeout).await {
            println!("{} is open", port);
        }
    }

    println!("Finished in {:?} seconds", instant.elapsed());
}