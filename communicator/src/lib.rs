pub mod client;

pub mod network;


#[cfg(test)]
mod tests {
    use client;
    use network::server;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_client() {assert_eq!(client::connect(), 0)}

    #[test]
    fn test_server() {assert_ne!(server::connect(), -1)}

//    another way to address root module without a 'use' declaration is using 'super'
    #[test]
    fn test_network() {assert_eq!(super::network::connect(), 0)}

}
