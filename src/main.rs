use std::{
    io::{self, Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
};

use simple_http::http::request::HttpRequest;

fn create_socket() -> SocketAddr {
    SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::LOCALHOST), 5500)
}

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let buf_str = String::from_utf8_lossy(&buffer);
    let request= HttpRequest::new(&buf_str)?;
    let response = request.response()?;

    println!("{:?}", &response);
    println!("{}", &response.response_body);

    let body = response.response_body.clone();



    stream.write(&mut body.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn serve(socket: SocketAddr) -> io::Result<()> {
    let listner = TcpListener::bind(socket)?;
    let mut counter = 0;
    for stream in listner.incoming() {
        match std::thread::spawn(|| handle_client(&mut stream?)).join() {
            Ok(_) => {
                counter += 1;
                println!("connected stream... {}", counter);
            }
            Err(_) => continue,
        };
    }
    Ok(())
}
fn main() -> io::Result<()> {
    let socket = create_socket();
    serve(socket)?;
    Ok(())
}