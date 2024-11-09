//! Simple example Rust app which sends CoT XML messages.
//! Goal is to put a dot on the map for a TAK/ATAK receiver,
//! using a simple UDP sender.

use clap::Parser;
use cot_proto::base::Cot;
use cot_proto::tak::create::DEFAULT_COT_TYPE_MARKER;
use cot_proto::tak::detail::TakMarkerDetail;

#[derive(Parser, Debug)]
struct Command {
    #[arg(short, long, default_value = "239.2.3.1")]
    dest_addr: String,
    #[arg(short = 'p', long, default_value_t = 6969)]
    dest_port: u16,
    #[arg(short, long)]
    callsign: String,
    #[arg(long)]
    lat: f64,
    #[arg(long)]
    lon: f64,
    #[arg(long, default_value_t = 0.0)]
    hae: f32,
    #[arg(long, default_value_t = 10.0)]
    ce: f32,
    #[arg(long, default_value_t = 10.0)]
    le: f32,
    #[arg(short = 't', long, default_value = DEFAULT_COT_TYPE_MARKER)]
    cot_type: String,
}

fn main() -> std::io::Result<()> {
    let args = Command::parse();
    let dest = format!("{}:{}", args.dest_addr, args.dest_port);
    let udp = std::net::UdpSocket::bind("0.0.0.0:0")?;

    let mut cot: Cot<TakMarkerDetail> = Default::default();
    cot.detail.contact.callsign = args.callsign;
    cot.point.lat = args.lat;
    cot.point.lon = args.lon;
    cot.point.hae = args.hae;
    cot.point.ce = args.ce;
    cot.point.le = args.le;

    let text = quick_xml::se::to_string(&cot).unwrap();
    udp.send_to(text.as_bytes(), dest.clone())?;
    println!("[!] Sent to {dest}:\n{text}");

    Ok(())
}
