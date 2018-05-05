
// 'a lifetime explicitly requires both params and return value to have the same lifetime as 'a (or longer)
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str{
    match s1.len() > s2.len() {
        true => return s1,
        _ => s2
    }
}

fn main() {
//    let g;
//
//    {
//        let l = 5;
//        g = &l; // g borrows l
//    } // won't compile - borrowed doesn't live long enough


    let x = 3;

    let r = &x;

    println!("{}", r);

    println!("{}", longest("longer than", "this"));

    let string1 = String::from("glob string");

    {
        let string2 = String::from("sec string");
        let result = longest(string1.as_str(), string2.as_str());
//        Valid - result doesn't live longer than any of the strings
    }

    let text1 = String::from("text");
    let result;
    {
        let text2 = String::from("more text");
//        result = longest(text1.as_str(), text2.as_str()); // won't compile text 2 doesn't live long enough
    }
    println!("res: {}", result);
}
