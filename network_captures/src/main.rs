extern crate pcap;
use pcap::Device;


extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ethernet::EtherTypes;


use std::env;

fn get_interface(interface_name: &str) -> NetworkInterface {
    let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    interfaces.into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap()
}


fn main() {

    let interface_name = Device::lookup().unwrap().name;
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
                let ethernet_packet = EthernetPacket::new(packet).unwrap();
                match ethernet_packet.get_ethertype() {
                    EtherTypes::Ipv4 => println!("ipv4"),
                    _ => {}
                }


            },
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
