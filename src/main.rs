use std::{str, io::Read, net::{TcpListener, TcpStream}};
use http_lib::HttpRequest;


fn handle_client(mut stream: TcpStream) {
    loop {
        let mut request = [0; 4096];
        let size = stream.read(&mut request).unwrap();

        if size == 0 { continue };

        let request = str::from_utf8(&request[..size]).unwrap();

        handle_request(request);
    }
}

fn handle_request(request: &str) {
    let request: Result<HttpRequest, http_lib::error::HttpRequestParseError> = request.try_into();

    match request {
        Ok(request) => println!("{:?}", request),
        Err(err) => println!("{}", err)
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }

    Ok(())
}
