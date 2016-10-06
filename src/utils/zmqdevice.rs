extern crate zmq;

/// `SimpleDevice` contain only two sockets
/// `in_socket` will be used to receive messages
/// `out_socket` wiil be used to send back messages
///
/// `SimpleDevice` is used for very basics elements like forwarder or basic receiver
pub struct SimpleDevice {
    in_socket: zmq::Socket,
    out_socket: zmq::Socket
}

impl SimpleDevice {

    pub fn new(in_socket: zmq::Socket, out_socket: zmq::Socket) -> SimpleDevice {
        SimpleDevice {
            in_socket: in_socket,
            out_socket: out_socket
        }
    }

    /// Run loop forever
    pub fn run(&mut self) {
        let mut msg = zmq::Message::new().unwrap();
        loop {
            self.in_socket.recv(&mut msg, 0).unwrap();
            println!("Received in message {}", msg.as_str().unwrap());
            self.out_socket.send_str(msg.as_str().unwrap(), 0).unwrap();
        }
    }
}

pub struct WorkerDevice {
    pull_socket: zmq::Socket
}

trait ProcessData {
    fn process_data(&self, &mut zmq::Message);
}

impl ProcessData for WorkerDevice {
    fn process_data(&self, msg: &mut zmq::Message) {}
}

impl WorkerDevice {

    pub fn new(pull_address: &str) -> WorkerDevice {
        let mut ctx = zmq::Context::new();

        let mut pull_socket = match ctx.socket(zmq::PULL) {
            Ok(pull_socket) => { pull_socket },
            Err(e) => { panic!(e) }
        };

        match pull_socket.bind(pull_address) {
            Ok(()) => (),
            Err(e) => panic!(e)
        }

        WorkerDevice {
            pull_socket: pull_socket
        }
    }

    pub fn run(&mut self) {
        let mut msg = zmq::Message::new().unwrap();
        loop {
            self.pull_socket.recv(&mut msg, 0).unwrap();
            self.process_data(&mut msg);
        }
    }
}
