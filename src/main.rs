use std::fs::read_to_string;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let thread_pool = ThreadPool::new(4);

    for result in listener.incoming() {
        let stream = result.unwrap();

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting Down!")
}

fn handle_connection(mut tcp_stream: TcpStream) {
    let reader = BufReader::new(&mut tcp_stream);
    let request = reader.lines().next().unwrap().unwrap();

    let (response_status, page) = match &request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index"),
        "GET /slow HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404"),
    };

    let contents = read_to_string(format!("static/{page}.html")).unwrap();

    let response = String::from(format!("{response_status}\r\n\r\n{contents}\r\n"));

    tcp_stream.write_all(response.as_bytes()).unwrap();
}
