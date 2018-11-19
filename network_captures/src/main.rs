extern crate pcap;
extern crate pnet;
extern crate regex;

#[macro_use] extern crate structopt;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

use pcap::Device;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::{TcpPacket, MutableTcpPacket};
use pnet::packet::ethernet::EtherTypes;
use std::prelude::v1::Option::None;
use std::env;
use regex::Regex;
use std::str;
use pnet::packet::ip::IpNextHeaderProtocols::Tcp;
use pnet::packet::ip::IpNextHeaderProtocols::Udp;
use structopt::StructOpt;

fn get_interface(interface_name: &str) -> NetworkInterface {
    let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    interfaces.into_iter()
        .filter(interface_names_match)
        .next()
        .expect(&format!("interface not found: '{}'", interface_name))
}


fn get_ipv4_ethernet_packet(packet: &[u8]) -> Option<EthernetPacket>{
        let ethernet_packet = EthernetPacket::new(packet).unwrap();
                match ethernet_packet.get_ethertype() {
                    EtherTypes::Ipv4 => Some(ethernet_packet),
                    _ => None
                }
}

fn search_pattern(pattern: &str, payload: &str) {
    let re = Regex::new(pattern).unwrap();
    for cap in re.captures_iter(payload) {
        println!("found capture: {:?}", cap);
    }

}

fn search_in_tcp(ipv4_ethernet_packet: EthernetPacket, pattern: &str) {
    let header = Ipv4Packet::new(ipv4_ethernet_packet.payload());
    if let Some(header) = header {
            if header.get_next_level_protocol() == Tcp  {
                let tcp_packet = TcpPacket::new(header.payload()).expect("failed to create tcp packet");
                let src_ip = header.get_source();
                let src_port = tcp_packet.get_source();
                let dst_ip = header.get_destination();
                let dst_port = tcp_packet.get_destination();
                println!("tcp - from: {}:{} to: {}:{}", src_ip, src_port, dst_ip, dst_port);
                let tcp_payload = tcp_packet.payload();

                search_pattern(pattern, &String::from_utf8_lossy(&tcp_payload))
            }

        }
    }

custom_derive! {
    #[derive(Debug, PartialEq, EnumDisplay, EnumFromStr)]
    enum Sieve  {
        whitelist,
        blacklist
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "pcapgrep", about = "Run grep on a pcap file.")]
struct Opt {
    #[structopt(short = "d", long = "debug", help = "Activate debug mode")]
    debug: bool,
    #[structopt(short = "s", long = "sieve", help = "ip and port sieve mode (whitelist/blacklist)")]
    sieve: Option<Sieve>,
//    #[structopt(long = "format", help = "How to format output")]
//    format: FormatOutput,
    #[structopt(long = "pattern", help = "Pattern to search")]
    pattern: Option<String>,
    #[structopt(long = "ip", help = "IP to filter")]
    ip: Option<String>,
    #[structopt(long = "port", help = "Port to filter")]
    port: Option<u16>,
    #[structopt(long = "interface", help = "Port to filter")]
    interface: Option<String>,
}

fn main() {

    let opt = Opt::from_args();
    let interface_name = opt.interface.unwrap_or("lo".into()); //Device::lookup().unwrap().name;
    println!("capturing from network interface: {}", interface_name);
    let interface = get_interface(&interface_name);

    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };

    loop {


        match rx.next() {
            Ok(packet) => {
                match  get_ipv4_ethernet_packet(packet) {
                    Some(ether) => {
//                        println!("ether: {:?}", ether);
                        match opt.pattern.as_ref() {
                            Some(pattern) => search_in_tcp(ether, &pattern),
                            _ => {}
                        }
//                        println!("found ether!");
                        ;
                    },
                    None => continue
                }

            },
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
