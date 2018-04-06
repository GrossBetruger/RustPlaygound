extern crate reqwest;
extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::io::copy;
use std::fs::File;


fn calcSha() {
    let input = "digest me!";
    let mut sha = Sha256::new();
    sha.input_str(&input);
    println!("{}", sha.result_str());
}


fn main() {
    calcSha();

    let mut resp = reqwest::get("http://support.apple.com/downloads/DL1507/en_US/GarageBand6.0.5Update.dmg").unwrap();
    assert!(resp.status().is_success());


    let mut dest = File::create("tmpf").unwrap();
    copy(&mut resp, &mut dest);

}
