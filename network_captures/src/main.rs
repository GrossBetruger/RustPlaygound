extern crate pcap;
use pcap::Device;

fn main() {
    let mut cap = Device::lookup().unwrap().open().unwrap();

    println!("main device: {:?}", main_device);

    while let Ok(packet) = cap.next() {
        println!("received packet! {:?}", packet);
    }
}