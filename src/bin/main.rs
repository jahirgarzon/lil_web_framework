use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::{fs::read_to_string, io::prelude::*, net::Shutdown};

use hello::{create_headers, create_response, create_response_line, startie, Controller, Route};

fn main() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 7878);
    let routes = vec![
        Route {
            method: hello::Method::GET,
            path: String::from("funny"),
            controller: Controller(|mut s: TcpStream| {
                let page = read_to_string("funny.html").unwrap();
                let rl = create_response_line(200, "OK");
                let response = create_response(&rl, "", &page);
                
                s.write(response.as_bytes()).unwrap();
                s.flush().unwrap();
                s.shutdown(Shutdown::Both).unwrap();
            }),
        },
        Route {
            method: hello::Method::GET,
            path: String::from("funnymonkey"),
            controller: Controller(|mut s: TcpStream| {
                let page = read_to_string("funny_monkey.html").unwrap();
                let rl = create_response_line(200, "OK");
                let response = create_response(&rl, "", &page);
                s.write(response.as_bytes()).unwrap();
                s.flush().unwrap();
                s.shutdown(Shutdown::Both).unwrap();
            }),
        },
        Route {
            method: hello::Method::GET,
            path: String::from("hello"),
            controller: Controller(|mut s: TcpStream| {
                let page = read_to_string("hello.html").unwrap();
                let rl = create_response_line(200, "OK");
                let response = create_response(&rl, "", &page);
                s.write(response.as_bytes()).unwrap();
                s.flush().unwrap();
                s.shutdown(Shutdown::Both).unwrap();
            }),
        },
        Route {
            method: hello::Method::GET,
            path: String::from(""),
            controller: Controller(|mut s: TcpStream| {
                let page = read_to_string(".html").unwrap();
                let rl = create_response_line(200, "OK");
                let headers =
                    create_headers(vec![("Content-Length", page.len().to_string().as_str())]);
                let response = create_response(&rl, &headers, &page);

                s.write(response.as_bytes()).unwrap();
                s.flush().unwrap();
                s.shutdown(Shutdown::Both).unwrap();
            }),
        },
    ];

    startie(routes, socket, 4)
}
