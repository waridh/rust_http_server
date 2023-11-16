use std::net::{TcpListener, TcpStream};

struct TcpError;

impl std::fmt::Display for TcpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

fn main() {
    // Binding the listener to the address. Will act as the host
    let listener = TcpListener::bind("127.0.0.1:27272").unwrap();

    // Iterating over connection attempts
    for stream in listener.incoming() { // A single stream is one connection
        let stream = stream.unwrap();   // Failure might occur on failed conn
        process_request(stream);        // If a stream goes out of scope, the
                                        // Connection fails
    }
}

fn process_request(stream: TcpStream) {
    println!("Connection Established");
}
