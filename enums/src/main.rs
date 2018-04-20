
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


#[derive(Debug)]
enum Message {
    Quit,
//    move includes an anonymous struct
    Move{x: u16, y: u16},
    Write(String),
    ChangeColor(u8, u8, u8)
}


impl Message {
    fn print_enum(&self) {
        println!("{:?}", self);
    }
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Texas
}


fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {println!("Lucky Penny!"); 1},
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
          println!("this quarter is from: {:?}!", state);
            25
        }
    }
}


fn safe_increment(x: Option<i8>) {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    };
}


fn print_low(num: u8){
    match num {
        0 => println!("zero"),
        1 => println!("one"),
        2 => println!("two"),
        _ => ()
    }
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

    let message = Message::Write(String::from("calling"));
    message.print_enum();


    let a: i8 = 5;
    let b: Option<i8> = Some(3);
//    let sum = a + b; won't compile b is Option
    println!("b plus 1 is: {:?}", safe_increment(b));
    let none = safe_increment(None);
    println!("none safely incremented to {:?}", none);

    let dime = Coin::Dime;
    let penny = Coin::Penny;
    println!("{} cents are worth a dime!", value_in_cents(dime));
    println!("{} cent is in a penny", value_in_cents(penny));
    let quarter = Coin::Quarter(UsState::Texas);
    value_in_cents(quarter);

//    matches with default arm
    print_low(0);
    print_low(1);
    print_low(2);
    print_low(3);
    print_low(100);

    let some_pi = Some(3.14);
    let some_approximate_pi = Some(22./7.);
//    if let sugar
    if let Some(3.14) = some_approximate_pi {
//        doesn't print
        println!("pi!");
    }

    else { println!("not pi...") }

    if let Some(3.14) = some_pi {
//        prints
        println!("pi!");
    }

    else { println!("not pi...") }
}

