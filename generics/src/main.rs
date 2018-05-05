use std::fmt::Display;

trait Summarizable {
    fn summary(&self) -> String {
//        default implementation (not a must)
        String::from("(read more...)")
    }

    fn get_author(&self) -> &str;

    fn sound_byte(&self) -> String {
//        default implementations can use functions with no
//        default implementations!!!
        format!("(read more from {})", self.get_author())
    }

}

#[derive(Debug)]
struct NewsArticle {
    headline: String,
    author: String,
    location: String,
    content: String
}

#[derive(Debug)]
struct Tweet {
    username: String,
    likes: u64,
    retweets: u32,
    content: String
}

struct TabloidPost {
    content: String,
    clicks: u64,
    author: String
}


impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn get_author(&self) -> &str {
        &self.author
    }
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}, by {} \nliked: {} \nretweeted: {}",
        self.content, self.username, self.likes, self.retweets)
    }

        fn get_author(&self) -> &str {
        &self.username
    }
}

impl Summarizable for TabloidPost {
        fn get_author(&self) -> &str {
        &self.author
    }
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}


#[derive(Debug)]
struct Pair<T, U> {
    car: T,
    cdr: U
}

struct TypedPair<T> {
    car: T,
    cdr: T
}

impl <T> TypedPair<T> {
    fn new(first: T, last: T) -> Self {
        Self {
            car: first,
            cdr: last
        }
    }
}

impl <T: Display + PartialOrd> TypedPair<T> {
    fn cmp_and_print(&self){
        if self.car > self.cdr {
            println!("car is larger: {}", self.car)
        }
        else if  self.cdr > self.car {
            println!("cdr is larger: {}", self.cdr)
        }
        else { println!("pair is symmetric!") }
    }
}

impl <T, U> Pair<T, U> {
    fn car(self) -> T {
        self.car
    }

    fn cdr(self) -> U {
        self.cdr
    }
}


impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

}

impl <T, U> WeakPoint<T, U> {
    fn mixup<V, W> (self, other: WeakPoint<V, W>) -> WeakPoint<T, W> {
        WeakPoint {x: self.x, y: other.y}
    }
}

impl Point<f32> {
    fn hipot(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

}

#[derive(Debug)]
struct WeakPoint<T, U> {
    x: T,
    y: U
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for n in list.iter() {
        if *n > largest {
            largest = *n;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for n in list.iter() {
        if *n > largest {
            largest = *n
        }
    }
    largest
}

fn generic_largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];
    for &n in list.iter() {
        if n > largest {
            largest = n;
        }
    }
    largest
}

fn notify<T: Summarizable>(post: T) {
    println!("this just in: {}", post.summary());
}

fn main() {
    let mut l = vec![63, 31, 15, 7];
    println!("largest val: {}", largest(&l));

    l.push(65535);
    println!("largest val: {}", largest(&l));

    let char_list = vec!['c', 'f', 'x', 'd'];
    println!("largest char: {}", largest_char(&char_list));
    println!();
    println!("generic largest: {}", generic_largest(&char_list));
    println!("generic largest: {}", generic_largest(&l));
    println!();

    let float_point = Point {x: 5.1, y: -3.33};
    let int_point = Point {x: 7, y: 7};
    println!("point {:?} has x of {}", int_point, int_point.x());
    println!("point {:?} distance from origin: {}", float_point, float_point.hipot());
//    let bad_point = Point {x: 2, y: 8.88}; // different types, won't compile
    let week_type_point = WeakPoint {x: 1, y: 0.01}; // that's fine - two generic types

    let signed_unsigned_i = WeakPoint {x: 2147483648i32, y: 2147483648u32};
    println!("ingedient 1: {:?}", signed_unsigned_i);
    let signed_unsigned_b = WeakPoint {x: 255i8, y: 255u8};
    println!("ingedient 2: {:?}", signed_unsigned_b);
    let mixed_point = signed_unsigned_i.mixup(signed_unsigned_b);
    println!("points mixup: {:?}", mixed_point);

    let lispy_pair = Pair {car: 3, cdr: 2};
    println!("simple lispy pair: {:?}", lispy_pair);
    let nested_lispy_pair = Pair {car: 2, cdr: Pair{car: 1, cdr: "nill"}};
    let inner_pair = nested_lispy_pair.cdr();
    println!("nested pair cdr: {:?}", inner_pair);

    let headline = String::from("The 40 most breathtaking lines from Donald Trump's NRA speech");
    let author = String::from("Chris Cillizza");
    let location = String::from("US");
    let content = String::from("(CNN)On Friday, President Donald Trump traveled to Texas to address the annual convention of the National Rifle Association.
        The speech capped a week that included revelations about the questions special counsel Robert Mueller wanted to ask Trump, allegations that Trump had dictated a note to his doctor about his stellar health in 2016, Rudy Giuliani's controversy-causing comments about the payout to porn star Stormy Daniels and the resignation of (yet another)
        member of Trump's legal team.");
    let news = NewsArticle {
        headline: headline, author: author,
        location: location, content: content
    };

    println!("\nnews summary:\n{}", news.summary());

    let content = String::from("First, it’s important to know that Tesla is the most shorted (meaning most bet against) stock on the market & has been for a while");
    let likes = 9500;
    let retweets = 933;
    let username = String::from("@elonmusk");
    let tweet = Tweet {
        username: username,
        content: content,
        retweets: retweets,
        likes: likes
    };

    println!("\ntweet summary:\n{}", tweet.summary());

    let tablo = TabloidPost {
        content: String::from("celebrities use this weird trick to..."),
        clicks: 18446744073709551615,
        author: String::from("Sensational Jane Doe")
    };

    println!("\ntabloid summary: {}", tablo.summary());
    println!("\ntabloid soundbyte: {}", tablo.sound_byte());
    println!("\ntweet soundbyte: {}", tweet.sound_byte());

    println!();
    // notify is trait-bounded to Summarizables
    notify(tweet);

    let byte_pair = TypedPair::new(255u8, 0u8);
    byte_pair.cmp_and_print();
    let byte_pair = TypedPair::new(256u8, 255u8); // car overflows
    byte_pair.cmp_and_print();

}
