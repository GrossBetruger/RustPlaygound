fn main() {
     let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = {if number == 3 {number + 1} else {number}};

    println!("updated number: {}", number);

    let a = [7, 8, 3, 2];

    for item in a.iter(){
        println!("item {}", item);
    }

    for (i, item) in a.iter().enumerate(){
        println!("item num {}: {}", i, item);
    }

    for i in (1..4).rev(){
        println!("{}", i);
    }
    println!("LIFTOFF...!");
}
