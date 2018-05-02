use std::io;
use std::io::Read;
use std::fs::File;

fn open_or_fail() -> File {
    File::open("not_here").unwrap()
}

fn open_or_fail_verbosly() -> File {
    File::open("not_here").expect("I was really expecting the file to be here, oh shrugs...")
}

//fn read_last_word_from_file() -> Result<String, io::Error> {
//    let f = File::open("words");
//    match f {
//        Ok(file) => file,
//        Err(e) => Err(e),
//    }
//
//    let mut s = String::new();
//
//    match f.read_to_string(&mut s) {
//        Ok(s) => s,
//        Err(e) => Err(e)
//    }
//}

fn read_text_from_file() -> Result<String, io::Error> {
    let f = File::open("words");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_text_from_file_sugar() -> Result<String, io::Error> {
    let mut f = File::open("words")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}


fn main() {

    let text = read_text_from_file();
    println!("{}", text.unwrap());
    let sugar_text = read_text_from_file_sugar();
    println!("{}", sugar_text.unwrap());
    open_or_fail_verbosly();
    open_or_fail();

    let f = File::open("critical_file");

    match f {
        Ok(file) => file,
        Err(ref error) => {
                match File::create("critical_file") {
                    Ok(fc) => fc,
                    Err(e) => {
                        panic!("tried to create critical_file, failed due to: {:?}", e.kind())
                }
            }
        },

        Err(error) => panic!("there was a problem opening the file {:?}", error)
    };

    let v = vec![3, 3, 1];
    let n = v[99];
    panic!("halt and catch fire!")


}
