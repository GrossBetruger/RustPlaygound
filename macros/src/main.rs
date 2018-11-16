

fn main() {
    #[macro_export]
    macro_rules! vec {
        ( $( $x:expr),* ) => {
            {
               let mut vec = Vec::new();
                $(
                    vec.push($x);
                )*
                vec
            }

        };
    }


    println!("macro created vec: {:?}", vec![1,2,3])
}
