use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::{fs::read_to_string};

use hello::{Controller, Route, send_downstream, startie};

fn main() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 7878);
    let routes = vec![
        Route {
            method: hello::Method::GET,
            path: String::from("funny"),
            controller: Controller(|s: TcpStream| {
                let page = read_to_string("pages/funny.html").unwrap();
                send_downstream(s, &page, None);
            }),
        },
        Route {
            method: hello::Method::GET,
            path: String::from("funnymonkey"),
            controller: Controller(|s: TcpStream| {
                let page = read_to_string("pages/funny_monkey.html").unwrap();
                send_downstream(s, &page, None);
            }),
        },
        Route {
            method: hello::Method::GET,
            path: String::from("hello"),
            controller: Controller(|s: TcpStream| {
                let page = read_to_string("pages/hello.html").unwrap();
                send_downstream(s, &page, None);
            }),
        },
        Route {
            method: hello::Method::GET,
            path: String::from(""),
            controller: Controller(|s: TcpStream| {
                let page = read_to_string("pages/.html").unwrap();
                send_downstream(s, &page, None);
            }),
        },
    ];

    startie(routes, socket, 4)
}
