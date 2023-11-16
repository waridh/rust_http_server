use std::{
    io::{prelude::*, BufReader},    // io::prelude is just the IO heavy modules
    net::{TcpListener, TcpStream},
};

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

fn process_request(mut stream: TcpStream) {
    // We want to buffer the reads from stream to improve performance
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader.lines()
        .map(|ele| ele.unwrap())    // The line is wrapped
        .take_while(|line| !line.is_empty())    // We can stop iterating
        .collect();                             // Convert back into vector
    println!("HTTP requests: {http_request:#?}");
}

#[cfg(test)]
mod test {
    #[test]
    fn place_holder() {
        assert!(true);
    }
}
