

fn main() {
    #[macro_export]
    macro_rules! vector {
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

    // Higher Order Macro:
    macro_rules! printlines {
        ($ ($x:expr),*) => {
            {
                $(println!("{}", $x);)*
            }
        };
    }


    println!("macro created vec: {:?}", vector![1,2,3]);

    printlines!("line: 1", "line: 2", "line: 3");


}
