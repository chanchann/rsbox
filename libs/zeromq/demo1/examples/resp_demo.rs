use zmq::{Context, Error, Socket};

fn main() -> Result<(), Error> {
    let context = Context::new();
    let responder = context.socket(zmq::REP)?;
    responder.bind("tcp://*:5555")?;

    loop {
        let message = responder.recv_msg(0)?;
        println!("Received request: {}", message.as_str().unwrap());

        responder.send("World", 0)?;
    }
}