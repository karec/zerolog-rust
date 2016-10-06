//! # Zerolog library
//!
//! Contains all functions used by zerolog to start devices
extern crate zmq;

pub mod utils;


use utils::zmqdevice::SimpleDevice;


/// Create sockets, binds them and call run method on `SimpleDevice`
///
/// Note that `run` method on `SimpleDevice` is blocking, so calling `start_forwarder`
/// will block
///
/// # Example
/// ```
/// start_forwarder("tcp://*:9001", "tcp://*:9002", "mytopic");
/// ```
pub fn start_forwarder(in_address: &str, out_address: &str, topic: &str) {
    let mut ctx = zmq::Context::new();

    let mut in_socket = match ctx.socket(zmq::SUB) {
        Ok(in_socket) => { in_socket },
        Err(e) => { panic!("Cannot create in_socket for forwarder. Error : {:?}", e)}
    };

    match in_socket.set_subscribe(topic.as_bytes()) {
        Ok(()) => (),
        Err(e) => panic!("Cannot subscribe to topic. Error : {:?}", e)
    }

    match in_socket.bind(in_address) {
        Ok(()) => (),
        Err(e) => panic!("Cannot bind in socket. Error : {:?}", e)
    }

    let mut out_socket = match ctx.socket(zmq::PUB) {
        Ok(out_socket) => { out_socket },
        Err(e) => { panic!(e) }
    };

    match out_socket.bind(out_address) {
        Ok(()) => (),
        Err(e) => panic!("Cannot bind out socket. Error: {:?}", e)
    }

    let mut device: SimpleDevice = SimpleDevice::new(in_socket, out_socket);
    println!("Starting forwarder.\nIn socket binded on {}\nOut socket binded on {}", in_address, out_address);
    device.run()
}
