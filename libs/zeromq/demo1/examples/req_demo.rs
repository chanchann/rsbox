use zmq::{Context, Error, Socket};

fn main() -> Result<(), Error> {
    let context = Context::new();
    let requester = context.socket(zmq::REQ)?;
    requester.connect("tcp://localhost:5555")?;

    for request_nbr in 0..10 {
        println!("Sending request {}...", request_nbr);
        requester.send("Hello", 0)?;

        let message = requester.recv_msg(0)?;
        println!("Received reply {}: {}", request_nbr, message.as_str().unwrap());
    }

    Ok(())
}