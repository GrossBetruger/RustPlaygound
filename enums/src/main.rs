
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}


struct IpAddr {
    kind: IpAddrKind,
    addr: String
}


#[derive(Debug)]
enum Ip {
    V4(String),
    V6(String)
}


#[derive(Debug)]
enum IpStrict {
    V4(u8, u8, u8, u8),
    V6(String)
}


fn main() {
    let my_ip_type = IpAddrKind::V4;
    println!("my ip type is: {:?}", my_ip_type);
    let advanced_user_ip_type = IpAddrKind::V6;
    println!("advanced user ip type: {:?}", advanced_user_ip_type);

    let my_ip = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("10.0.0.1")
    };

    let another_ip = IpAddr {kind: IpAddrKind::V6, addr: String::from("127.0.0.1")};

    println!("my ip: {:?}, {}", my_ip.kind, my_ip.addr);
    println!("another ip: {}", another_ip.addr);

    let home = Ip::V4(String::from("127.0.0.1"));
    let loopback = Ip::V6(String::from("::1"));

    println!("home ip: {:?}", home);
    println!("loopback ip: {:?}", loopback);

    let strict_ip = IpStrict::V4(255, 255, 255, 255);

}
