use std::{fs::read_to_string, io::prelude::*};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};

use hello::{Controller, Route, create_response, create_response_line, startie};

fn main() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 7878);
    let routes = vec![
         Route{
            method:hello::Method::GET,
            path:String::from("funny"),
            controller:Controller(|mut s:TcpStream|{
                let page = read_to_string("funny.html").unwrap();
                let rl =create_response_line(200, "OK");
                let response = create_response(&rl, &page);
                s.write(response.as_bytes()).unwrap();
                s.flush().unwrap();
            })
        },
         Route{
            method:hello::Method::GET,
            path:String::from("funnymonkey"),
            controller:Controller(|mut s:TcpStream|{
                let page = read_to_string("funny_monkey.html").unwrap();
                let rl =create_response_line(200, "OK");
                let response = create_response(&rl, &page);
                s.write(response.as_bytes()).unwrap();
                s.flush().unwrap();
            })
        },
         Route{
            method:hello::Method::GET,
            path:String::from("hello"),
            controller:Controller(|mut s:TcpStream|{
                let page = read_to_string("hello.html").unwrap();
                let rl =create_response_line(200, "OK");
                let response = create_response(&rl, &page);
                s.write(response.as_bytes()).unwrap();
                s.flush().unwrap();
            })
        },
        Route{
            method:hello::Method::GET,
            path:String::from(""),
            controller:Controller(|mut s:TcpStream|{
                let page = read_to_string(".html").unwrap();
                let rl =create_response_line(200, "OK");
                let response = create_response(&rl, &page);
                s.write(response.as_bytes()).unwrap();
                s.flush().unwrap();
            })
        }
    ];

    startie(routes, socket, 4)
}
