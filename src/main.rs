use std::{
    fs, // File system library. Can load files into memory
    io::{prelude::*, BufReader},    // io::prelude is just the IO heavy modules
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use kiki_delivery_server::ThreadPool;

struct TcpError;

impl std::fmt::Display for TcpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

fn main() {
    // Binding the listener to the address. Will act as the host
    let listener = TcpListener::bind("127.0.0.1:27272").unwrap();
    let pool = ThreadPool::new(4);

    // Iterating over connection attempts
    for stream in listener.incoming() { // A single stream is one connection
        let stream = stream.unwrap();   // Failure might occur on failed conn
        pool.execute(|| {process_request(stream);});
    }
}

fn process_request(mut stream: TcpStream) {
    // We want to buffer the reads from stream to improve performance
    let buf_reader = BufReader::new(&mut stream);

    let http_request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = match &http_request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "kiki_landing.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));  // To get a block to occur
            ("HTTP/1.1 200 OK", "kiki_landing.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(file_name).unwrap();
    stream.write_all(response_generator(status_line, contents).as_bytes())
        .unwrap();

}

fn response_generator(status_line: &str, content: String) -> String {
    let length = content.len();
    let ret = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    // function testing for response generation
    #[test]
    fn test_response_generator() {
        let expected_output1 = String::from("HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n");
        assert_eq!(response_generator("HTTP/1.1 200 OK", String::from("")), expected_output1);
    }
}
