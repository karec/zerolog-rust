extern crate zerolog;
extern crate zmq;

use zerolog::utils::zmqdevice::SimpleDevice;

#[test]
fn basic_test() {
    assert!(1 == 1);
}

#[test]
fn can_create_simple_device() {

    let mut ctx = zmq::Context::new();

    let mut in_socket = match ctx.socket(zmq::SUB) {
        Ok(in_socket) => { in_socket },
        Err(e) => { panic!(e) }
    };

    match in_socket.bind("tcp://*:0") {
        Ok(()) => (),
        Err(e) => panic!(e)
    }

    let mut out_socket = match ctx.socket(zmq::REQ) {
        Ok(out_socket) => { out_socket },
        Err(e) => { panic!(e) }
    };

    match out_socket.bind("tcp://*:0") {
        Ok(()) => (),
        Err(e) => panic!(e)
    }

    let device = SimpleDevice::new(in_socket, out_socket);
}

