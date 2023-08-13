use std::net::{SocketAddr, UdpSocket};

use clap::Parser;
use dns_weekend::{query::Query, question::QType};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The domain name to resolve
    #[clap(short, long)]
    addr: String,
    #[clap(long, default_value_t = String::from("1.1.1.1"))]
    dns_addr: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let query: Vec<u8> = Query::build_query(&args.addr, QType::A);

    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let remote_addr: SocketAddr = format!("{}:53", &args.dns_addr).parse()?;
    println!("{:02X?}", query);
    socket.send_to(&query, remote_addr)?;
    println!("Sent data to {}", remote_addr.to_string());

    let mut buffer = [0u8; 1024];
    let (size, _) = socket.recv_from(&mut buffer)?;

    println!("Received {} bytes: {:02X?}", size, &buffer[..size]);

    return Ok(());
}
