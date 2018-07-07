//! # art
//!
//!
//! a library that expresses artistic concepts



pub mod kinds {

    #[derive(Debug)]
    #[derive(PartialEq)]
    #[derive(Eq)]
    #[derive(Ord)]
    #[derive(PartialOrd)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    #[derive(Eq)]
    #[derive(Ord)]
    #[derive(PartialOrd)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }

}

pub mod utils {
    use kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) ->  SecondaryColor {
        let mut colors = vec![c1, c2];
        colors.sort();

        let red_yellow = vec![PrimaryColor::Red, PrimaryColor::Yellow];
        match colors {
            red_yellow => {
                SecondaryColor::Orange
            },

            _ => SecondaryColor::Purple
        }

    }
}

#[cfg(test)]
mod tests {


    #[test]
    fn it_works() {

        use super::kinds::*;
        use super::utils::*;

        let red = PrimaryColor::Red;
        let yellow = PrimaryColor::Yellow;
        assert_eq!(mix(yellow, red), SecondaryColor::Orange);

        let red = PrimaryColor::Red;
        let yellow = PrimaryColor::Yellow;

        assert_eq!(mix(red, yellow), SecondaryColor::Orange)

    }
}
