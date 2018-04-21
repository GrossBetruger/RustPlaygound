extern crate encapsulation;

use encapsulation::super_nested::nest_a::nest_b::*;
use encapsulation::TrafficLights::{Red, Yellow};
use encapsulation::TrafficLights;

fn main() {
    encapsulation::super_nested::nest_a::nest_b::execute_me();
    execute_me();
    dare_you_execute_me();
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLights::Green;
}