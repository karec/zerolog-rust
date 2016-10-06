extern crate zerolog;
extern crate zmq;

use zerolog::utils::zmqdevice::SimpleDevice;

fn main() {
    println!("Hello, world!");


    let mut ctx = zmq::Context::new();

    let mut in_socket = match ctx.socket(zmq::SUB) {
        Ok(in_socket) => { in_socket },
        Err(e) => { panic!(e) }
    };

    match in_socket.set_subscribe("".as_bytes()) {
        Ok(()) => (),
        Err(e) => panic!("Cannot subscribe to channel : {:?}", e)
    }

    match in_socket.bind("tcp://*:9001") {
        Ok(()) => (),
        Err(e) => panic!("Cannot bind in socket ! Error is {:?}", e)
    }

    let mut out_socket = match ctx.socket(zmq::PUB) {
        Ok(out_socket) => { out_socket },
        Err(e) => { panic!(e) }
    };

    match out_socket.bind("tcp://*:9002") {
        Ok(()) => (),
        Err(e) => panic!("Cannot bind out socket ! Error is {:?}", e)
    }

    let mut test: SimpleDevice = SimpleDevice::new(in_socket, out_socket);
    test.run()
}
