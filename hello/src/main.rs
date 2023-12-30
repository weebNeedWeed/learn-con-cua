use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::build(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }

    println!("Shutting down")
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = if http_request == "GET / HTTP/1.1" {
        ("HTTP/1.1 OK 200", "index.html")
    } else {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 404 Not Found", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let content_length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\nContent-Type: text/html\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap()
}
