use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::fs;

fn handle_client(mut stream:TcpStream){
    let mut buffer=[0;1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}",String::from_utf8_lossy(&buffer[..]));

    let get=b"GET / HTTP/1.1\r\n";

    let (status_line,filename)=if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n","ok.html")
    }else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n","404.html")
    };
    // let contents=fs::read_to_string(filename).unwrap();
    // let response=format!("{}{}",status_line,contents);
    let response=format!("{}{}",status_line,fs::read_to_string(filename).unwrap());
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn main()->std::io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:2333")?;

    for stream in listener.incoming(){
        handle_client(stream?);
    }
    Ok(())
}
