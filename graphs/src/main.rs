extern crate petgraph;
extern crate pnet;
extern crate pcap;
#[macro_use] extern crate structopt;

use petgraph::Graph;
use petgraph::dot::{Dot, Config};

use pcap::Error as PcapError;
use pnet::packet::Packet;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::datalink::pcap as dlpcap;
use pnet::datalink::Channel;
use std::path::PathBuf;
use structopt::StructOpt;
use std::collections::HashSet;
use petgraph::prelude::NodeIndex;
use std::collections::HashMap;
use std::net::Ipv4Addr;

#[derive(Debug, StructOpt)]
#[structopt(name = "pcap_graph", about = "Create graph from a pcap file.")]
struct Opt {
    #[structopt(help = "Pcap files", parse(from_os_str))]
    pcaps: Vec<PathBuf>,
}


fn parse_pcaps(pcaps: Vec<PathBuf>) -> HashMap<Ipv4Addr, Ipv4Addr> {

       let mut unique_edges = HashMap::new();
       for pcap_path in pcaps {
        match dlpcap::from_file(&pcap_path, Default::default()) {
            Ok(Channel::Ethernet(_tx, mut rx)) => {
                let mut pcap_pkt_counter = 0;
                loop {
                    pcap_pkt_counter += 1;
                    match rx.next() {
                        Ok(pktbuf) => {

                            let eth_pkt = EthernetPacket::new(pktbuf).unwrap();
                            match Ipv4Packet::new(eth_pkt.payload()) {
                                Some(res) => {
                                    let src_ip = res.get_source();
                                    let dst_ip = res.get_destination();
                                    unique_edges.insert(src_ip, dst_ip);

                                },
                                _ => {
                                    eprintln!("ERROR NO IP");
                                }
                            };


                        },

                        Err(e) => {
                            return unique_edges;
                        }
                    };
                }
            },
            Err(e) => {
                panic!("Error opening pcap, path: {} err: {:?}", pcap_path.display(), e);
            },
            _ => {
                panic!("Oh no!");
            }
        }
    }
    unique_edges

}


fn main() {
    let opt = Opt::from_args();

    println!(" OPT = {:?}", opt);

    let mut g = Graph::<&str, i32, petgraph::Undirected>::new_undirected();

    println!("parsing packets...");
    let unique_edges = parse_pcaps(opt.pcaps);
    println!("Done parsing!");
    for (src_ip, dst_ip) in unique_edges {
        println!("{}, {}", src_ip, dst_ip);
    }

//     Create a new undirected graph, g
//    let w = 0;
//    let mut v = Vec::new();

//    // Add 10 vertices to G
//    for i in 1..11 {
//        v.push(g.add_node("some_ip"));
//    }
//
//    // Connect with 15 edges
//    for i in 0..4 {
//        g.add_edge(v[i], v[i + 1], w);
//        g.add_edge(v[i], v[i + 5], w);
//    }
//    g.add_edge(v[0], v[4], w);
//    g.add_edge(v[4], v[9], w);
//
//    g.add_edge(v[5], v[7], w);
//    g.add_edge(v[5], v[8], w);
//    g.add_edge(v[6], v[8], w);
//    g.add_edge(v[6], v[9], w);
//    g.add_edge(v[7], v[9], w);
//
//    // Print in graphviz dot format
//    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
}