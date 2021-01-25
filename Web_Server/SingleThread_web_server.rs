use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::fs;

fn handle_client(mut stream:TcpStream){
    let mut buffer=[0;1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}",String::from_utf8_lossy(&buffer[..]));

    let get=b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {

        let content=fs::read_to_string("ok.html").unwrap();
        let response=format!("HTTP/1.1 200 OK\r\n\r\n{}",content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }else {
        let content=fs::read_to_string("404.html").unwrap();
        let response =format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}",content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

pub fn main()->std::io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:2333")?;

    for stream in listener.incoming(){
        handle_client(stream?);
    }
    Ok(())
}
