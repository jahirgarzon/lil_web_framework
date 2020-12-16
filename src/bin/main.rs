use hello::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_conn(stream);
        });
    }
}

fn handle_conn(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let paths = vec!["funny", "funny_monkey", "hello"];
    let lines: Vec<Result<String, _>> = buffer.lines().collect();
    let mut first_line = lines.get(0).unwrap().as_ref().ok().unwrap().split(" ");
    let verb = first_line.next().unwrap();
    let path = first_line.next().unwrap();
    println!("path: {}", path);
    let response = match path {
        "/" => Some(""),
        _ => paths.into_iter().find(|p| {
            let mut pattern = String::from("/");
            pattern.push_str(p.to_owned());
            return path.starts_with(pattern.as_str());
        }),
    };

    let a = match response {
        Some("/") => ("HTTP/1.1 200 OK\r\n\r\n", String::from(".html")),
        Some(good_path) => {
            let mut good_path = good_path.to_string();
            good_path.push_str(".html");
            ("HTTP/1.1 200 OK\r\n\r\n", good_path)
        }
        None => (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            "notfound.html".to_string(),
        ),
    };

    let (status_line, filename) = a;
    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
