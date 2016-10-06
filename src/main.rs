extern crate zerolog;
extern crate zmq;

use zerolog::start_forwarder;

fn main() {
    println!("Starting forwarder");
    start_forwarder("tcp://*:9001", "tcp://*:9002", "");
}
