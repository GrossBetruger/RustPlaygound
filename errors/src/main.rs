use std::fs::File;

fn main() {

    let f = File::open("critical_file");

    match f {
        Ok(file) => file,
        Err(ref error) => {
                match File::create("critical_file") {
                    Ok(fc) => fc,
                    Err(e) => {
                        panic!("tried to create critical_file, failed due to: {:?}", e)
                }
            }
        },

        Err(error) => panic!("there was a problem opening the file {:?}", error)
    };

    let v = vec![3, 3, 1];
    let n = v[99];
    panic!("halt and catch fire!")
}
