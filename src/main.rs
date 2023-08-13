use std::net::{SocketAddr, UdpSocket};

use clap::Parser;
use dns_weekend::{query::Query, question::QType};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The domain name to resolve
    #[clap(short, long)]
    address: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let query: Vec<u8> = Query::build_query(&args.address, QType::A);

    let addr = "127.0.0.1:12345";
    let socket = UdpSocket::bind(addr)?;
    println!("Listening on {}", addr);

    let remote_addr: SocketAddr = "8.8.8.8:53".parse()?;
    println!("{:?}", query);
    socket.send_to(&query, remote_addr)?;
    println!("Sent data to {}", remote_addr.to_string());

    let mut buffer = [0u8; 1024];
    let (size, sender_addr) = socket.recv_from(&mut buffer)?;

    println!(
        "Received {} bytes from {}: {}",
        size,
        sender_addr,
        String::from_utf8_lossy(&buffer[..size])
    );

    return Ok(());
}
