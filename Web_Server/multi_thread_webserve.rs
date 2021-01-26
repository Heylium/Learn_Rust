use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{thread, time};
use std::fs;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "ok.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    // let contents=fs::read_to_string(filename).unwrap();
    // let response=format!("{}{}",status_line,contents);
    let response = format!("{}{}", status_line, fs::read_to_string(filename).unwrap());
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    //线程睡眠一小段时间
    let ten_milsec = time::Duration::from_millis(10000);
    thread::sleep(ten_milsec);
}

pub fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2333")?;
    let mut thread_vec:Vec<thread::JoinHandle<()>>=Vec::new();

    //在stream中每收到一个请求，就新建一个线程，并加入到容器vec中
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let handle=thread::spawn(move || {
            handle_client(stream);
        });
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
}
