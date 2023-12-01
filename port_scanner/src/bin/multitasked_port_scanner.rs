use clap::Parser;
use std::time::Instant;
use port_scanner::{is_open_async, Options};
use port_scanner::is_open_sync;

#[tokio::main]
async fn main() {
    let instant = Instant::now();
    let options = Options::parse();
    let mut handles = vec![];
    for port in options.port_min..options.port_max {
        let host = options.host.clone();
        let thread = tokio::spawn(async move  {
            if is_open_async(&host, port, options.timeout).await {
                println!("Port {} is open", port);
            } else {
                println!("Port {} is closed", port);
            }
        });

        handles.push(thread);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    println!("Time elapsed: {:?}", instant.elapsed());
}