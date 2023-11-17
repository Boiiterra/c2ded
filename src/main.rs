use rtshark;

fn main() {
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
                println!("\t{}", metadata.display());
            }
        }
        println!("\n---------------------------\n");
    }
}
