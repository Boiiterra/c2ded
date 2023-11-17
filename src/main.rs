use rtshark;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

fn main() {
    let mut ips4: Vec<Ipv4Addr> = Vec::new();
    let mut ips6: Vec<Ipv4Addr> = Vec::new();

    let builder = rtshark::RTSharkBuilder::builder()
        .input_path("wlp4s0") // Wi-Fi
        // .input_path("enp3s0") // Ethernet
        .live_capture();

    // Start a new tshark process
    let mut rtshark = builder
        .spawn()
        .unwrap_or_else(|e| panic!("Error starting tshark: {e}"));

    // read packets until the end of the PCAP file
    while let Some(packet) = rtshark.read().unwrap_or_else(|e| {
        eprintln!("Error parsing tshark output: {e}");
        None
    }) {
        for layer in packet {
            println!("Layer: {}", layer.name());
            for metadata in layer {
                let value = metadata.name();
                // println!("\t{}", metadata.display());
                print!("\t'{}'", value);
                // // println!("\t{}", metadata.size());
                // // println!("\t'{}'", metadata.value());
                if value == "ip.src" {
                    let ip= IpAddr::from_str(metadata.value()).unwrap();
                    // let ip = metadata.value();
                    print!(" -> IP source: {};", ip);
                    print!(" ipv4 -> {};", ip.is_ipv4());
                    print!(" ipv6 -> {};", ip.is_ipv6());
                }

                if value == "ip.dst" {
                    let ip = IpAddr::from_str(metadata.value()).unwrap();

                    print!(" -> IP source: {};", ip);
                    print!(" ipv4 -> {};", ip.is_ipv4());
                    print!(" ipv6 -> {};", ip.is_ipv6());
                }
                println!("");
            }
        }
        println!("\n---------------------------\n");
    }
}
