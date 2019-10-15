/*
 * Traceroute utility: shows the route of a packet from your address to a given location.
 * Author: Kyle Jonson
 */

pub extern crate dns_lookup;
use dns_lookup::{lookup_host, lookup_addr};
use std::env;
use std::io::ErrorKind;
use std::net::{SocketAddr, UdpSocket};
use std::process;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if (args.len() == 1) {
        usage(&args[0]);
        process::exit(1);
    }
    // get host addr info
    let hostname = String::from(&args[1]);
    println!("Tracing route to: {}", hostname);
    let ips: Vec<std::net::IpAddr> = lookup_host(&hostname).unwrap();
    let addr = SocketAddr::new(ips[1], 33434);
    // create UDP socket
    let socket = UdpSocket::bind("127.0.0.1:33434")?;
    socket.set_nonblocking(true).expect("Failed to enter non-blocking mode");
    socket.set_read_timeout(Some(Duration::from_secs(10))).expect("set_read_timeout call failed");
    let mut hops: u32 = 30;

    if (args.len() > 2) {
        parse_args(args);
    }
    let mut buffer = [0; 30];
    // loop through ttl values to extend a hop for each packet
    // TODO: run these on threads and report back to an array
    for ttl in 1..hops {
        let mut visible_hop: bool = false;
        //set TTL and send packet
        socket.set_ttl(ttl).expect("set ttl call failed");
        socket.send_to(&[0; 1], addr);
        //receive diagnostic info
        let start = Instant::now();
        // receive packets for 3 seconds
        while start.elapsed().as_secs() < 3 {
            let result = socket.recv_from(&mut buffer);
            match result {
                Ok((num_bytes, src_addr)) => {
                    visible_hop = true;
                    let name = lookup_addr(&(src_addr.ip())).unwrap();
                    println!("{} {} ({})", ttl, name, src_addr.ip().to_string());
                },
                Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
                    println!("Something went wrong: {}", err)
                }
                _ => { }
            }
            // wait another 5ms for packets to arrive
            thread::sleep(Duration::from_millis(5));
        }
        if(!visible_hop) {
            println!("* * *");
        }
    }
    process::exit(0);
}

fn parse_args(args: Vec<String>) {
    //TODO: add arg options and parse them
}

fn usage(name: &String) {
    println!("Usage: ./{} address/host", name);
    println!("  Options:");
    println!("  -h num: Set max hops");
}