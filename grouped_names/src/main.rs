extern crate regex;

fn main() {
    let re = regex::Regex::new(r"(?P<lower>[0-5]+)|(?P<higher>[6-9]+)").unwrap();
    
    let mut groups = vec![];
    for name in re.capture_names() {
        if name.is_some() {
            groups.push(name.unwrap());
        }
    }
    
    re.is_match("22 96");
    
    let res: Vec<(String, u32)> = vec![];
    
    for cap in re.captures_iter("22 96") {
        let found: Vec<&str> = groups.iter().filter(|x| cap.name(x).is_some()).map(|x| *x).collect();
        println!("{:?}, offset {:?}", found, cap.get(0));
        
    }
}