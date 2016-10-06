extern crate zmq;


pub struct SimpleDevice {
    in_socket: zmq::Socket,
    out_socket: zmq::Socket,
}

impl SimpleDevice {

    pub fn new(in_socket: zmq::Socket, out_socket: zmq::Socket) -> SimpleDevice {
        SimpleDevice {
            in_socket: in_socket,
            out_socket: out_socket
        }
    }

    pub fn run(&mut self) {
        let mut msg = zmq::Message::new().unwrap();
        loop {
            self.in_socket.recv(&mut msg, 0).unwrap();
            println!("Received in message {}", msg.as_str().unwrap());
            self.out_socket.send_str(msg.as_str().unwrap(), 0).unwrap();
        }
    }
}
