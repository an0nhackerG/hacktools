#![allow(unused_imports)]
#![allow(unused_variables)]

use shared::logger::*;
use clap::{arg, Arg, Command};
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use std::sync::Arc;
use tokio::time::{timeout, Duration};

#[allow(dead_code)]
async fn example(){
    let target_ip = "192.168.122.130";
    let port_test: Vec<u16> = (1..=1000).collect();
    let sem = Arc::new(Semaphore::new(1000));

    let tasks: Vec<_> = port_test.into_iter().map(|port| {
        let sem_clone = sem.clone();
        let target_ip = target_ip.to_string();

        tokio::spawn(async move {
            let _permit = sem_clone.acquire().await;
            let addr = format!("{}:{}", target_ip, port);
            match timeout(Duration::from_secs(3), TcpStream::connect(&addr)).await {
                Ok(Ok(_conn)) => println!("Port: {} is open", port),
                Ok(Err(_)) => println!("Port: {} is closed", port),
                Err(_) => println!("Port: {} timed out", port),
            }
        })
    }).collect();

    for task in tasks {
        let _ = task.await;
    }
}

struct Target {
    ip: String,
    port: Vec<u16>
}

struct Scanner {
    target: Target
}

#[tokio::main]
async fn main() {
    let arguments = Command::new("ntwscan")
    .author("Luna")
    .about("A complete network scanner")
    .arg(
        Arg::new("scan_file")
        .short('f')
        .long("scan_file")
        .help("Yaml containing targets ips, ports")
    ).get_matches();
}

