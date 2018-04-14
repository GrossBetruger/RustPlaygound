#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn expand(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    fn can_hold(&self, rect_b: &Rectangle) -> bool {
        self.width >= rect_b.width && self.height >= rect_b.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}


fn area(width: u32, height: u32) -> u32 {
    width * height
}


fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}


fn main() {

    let height = 12;
    let width = 55;

    println!("the area of the rectangle is: {}", height * width);
    println!("the area of the rectangle is: {}", area(width, height));
    println!("the area of the rectangle is: {}", area2((width, height)));

    let mut r1 = Rectangle {
        width: 12,
        height: 55
    };

    println!("the area of the rectangle is: {}", area3(&r1));

    println!("rectangle 1: {:?}", r1);
    println!("rectangle 1 verbose print: {:#?}", r1);

    println!("the area of the rectangle is: {}", r1.area());
    r1.expand(2);
    println!("the width of the rectangle is: {}", r1.width);
    println!("the height of the rectangle is: {}", r1.height);
    println!("the area of the rectangle is: {}", r1.area());
    r1.expand(3);

    let r2 = Rectangle {
        height: 300,
        width: 70
    };

    let r3 = Rectangle {
        height: 350,
        width: 77
    };
    println!("the area of the rectangle is: {}", r1.area());
    println!("rect: {:?} can hold rect: {:?} is {}", r1, r2, r1.can_hold(&r2));
    println!("rect: {:?} can hold rect: {:?} is {}", r1, r3, r1.can_hold(&r3));

    let sq1 = Rectangle::square(10);
    println!("square 1: {:?}, area: {}", sq1, sq1.area());

}
