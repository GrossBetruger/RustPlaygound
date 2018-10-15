extern crate regex;

fn main() {
    let re = regex::Regex::new(r"(?P<lower>[0-5]+)|(?P<higher>[6-9]+)").unwrap();

    let mut groups = vec![];
    for name in re.capture_names() {
        if name.is_some() {
            groups.push(name.unwrap());
        }
    }

    for cap in re.captures_iter("22 96") {
        let found: Vec<&str> = groups.iter().filter(|x| cap.name(x).is_some()).map(|x| *x).collect();
        for (i, found_name) in found.into_iter().enumerate() {
            println!("{:?} {:?}", found_name, cap.get(i));
        }
    }
}