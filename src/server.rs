use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::io::Write;


// 处理客户端
fn handle_client(mut stream: TcpStream) {
    loop {
        let mut read = [0; 1028]; // 1024 byte buffer
        // 匹配读取数据
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 { 
                    // connection was closed
                    break;
                }
                // echo读到的数据
                stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                panic!("Error {}", err);
            }
        }
    }
}

fn main() {
    // 监听本地网络8080端口
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // 循环端口输入流
    for stream in listener.incoming() {
        // 匹配输入流数据
        match stream {
            // 模式匹配成功，执行handle_client
            Ok(stream) => { 
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            // 模式匹配错误
            Err(_) => {
                println!("Error");
            }
        }
    }
}