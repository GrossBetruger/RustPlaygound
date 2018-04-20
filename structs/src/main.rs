struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

// doesn't compile -- missing lifetime parameter for &str
//struct Borrower {
//    username: &str
//};


fn build_user_verbose(username: String, email: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 0
    }
}


fn build_user(username: String, email: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0
    }
}


fn increment_sign_in(mut user: User) -> User {
    user.sign_in_count += 1;
    user
}


struct QuitMessage; // unit struct


fn main() {

    let mut shimshon = User {
        email: String::from("dontscrapthisstupidbots@gmail.com"),
        sign_in_count: 1_000,
        active: true,
        username: String::from("Shimshon")
    };

    let dango = build_user(String::from("dango"), String::from("efes@gmail.com"));

//  initialize with fields copied from another users
    let gilad = User {
        email: dango.email.replace("efes", "1"),
        username: String::from("Gilad"),
        active: dango.active,
        sign_in_count: dango.sign_in_count
    };

    let avi = User {
        email: String::from("avi@mailbox.com"),
        username: String::from("Avi"),
        ..shimshon
    };


    println!("user name: {}", shimshon.username);
    println!("user sign-ins: {}", shimshon.sign_in_count);

//    increment field in mutable Struct
    shimshon = increment_sign_in(shimshon);
    println!("user name: {}", shimshon.username);
    println!("user sign-ins: {}", shimshon.sign_in_count);

    println!("user name: {}", dango.username);
    println!("user sign-ins: {}", dango.sign_in_count);

    println!("user: {}, email: {}", gilad.username, gilad.email);
    println!("user: {}, sign-ins: {}", avi.username, avi.sign_in_count);

//    Tuple Structs! (they're kinda like named-tuples)

    struct Color(u8, u8, u8);

    let black = Color(0, 0, 0);
    let white = Color(0xff, 0xff, 0xff);

    let r = white.0;
    let g = white.1;
    let b = white.2;
    println!("color white: rgb values: {},{},{}", r, g, b);


    struct Coords(i32, i32, i32);

    let origin = Coords(0, 0, 0);
    let down_100 = Coords(0, -100, 0);

}
