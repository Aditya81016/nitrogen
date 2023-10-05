// imports
use std::net::TcpListener;
use tungstenite::{accept, Message};

fn main() {
    // address for listener
    let addr = "0.0.0.0:8080";

    // creating a tcp listener
    let listener = TcpListener::bind(addr).unwrap();
    println!("Listening on: {}", addr);

    // handling incoming streams
    for stream in listener.incoming() {
        match stream {
            // if stream is ok
            Ok(stream) => {
                // try to connect to incoming stream
                let mut ws = accept(stream).expect("Failed to connect");
                println!("New connection: {:?}", ws.get_mut().local_addr().unwrap());

                // send confirmation message to client
                ws.send(Message::Text("Hello, Client!".to_string()))
                    .unwrap();

                // listen for messages while the stream is connected
                while ws.can_read() {
                    // get the message object
                    let msg = ws.read();

                    match msg {
                        Ok(msg) => match msg {
                            Message::Text(msg) => {
                                println!("Received: {:?}", msg);

                                // if incoming message is not confirmation message then...
                                if msg != "Hello, Server!" {
                                    // reply that the server received it
                                    ws.send(Message::Text(
                                        format!("I heard you say: {:?}", msg).to_string(),
                                    ))
                                    .unwrap();
                                }
                            }
                            _ => (),
                        },
                        // if listening to msg gave an error then...
                        Err(_) => {
                            // conclude that the client has disconnected
                            println!("Disconnected: {:?}", ws.get_mut().local_addr().unwrap());
                        }
                    }
                }
            }
            // handle error in incoming stream
            Err(err) => {
                println!("There was an error in incoming stream: \n {:?}", err)
            }
        }
    }
}
