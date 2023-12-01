use std::net::ToSocketAddrs;
use std::time::Duration;
use clap::Parser;

pub fn is_open_sync (host: &str, port: u16, timeout:u64) -> bool {
    let mut addrs_iter = format!("{}:{}", host, port).to_socket_addrs().unwrap();
    std::net::TcpStream::connect_timeout(&addrs_iter.next().unwrap(), Duration::from_secs(timeout)).is_ok()
}
#[derive(Parser)]
pub struct Options{
    pub host : String,
    pub port_min : u16,
    pub port_max : u16,
    pub timeout : u64
}