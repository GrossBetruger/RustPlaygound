extern crate adder;


#[test]
fn test_increment(){
    assert_eq!(0, adder::misc::increment(-1));
}