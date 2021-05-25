use std::io::{ self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

// 返回结果
fn main() -> std::io::Result<()> {
    // 建立连接
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    
    // 循环输入5次信息
    for _ in 0..5 {
        let mut input = String::new(); //定义可变字符串

        // 读取输入内容，如果错误输出错误信息
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        
        // 将值写入流传给服务器，如果错误输出错误信息
        stream
            .write(input.as_bytes())
            .expect("Failed to write to stream");
    
        // 打印输入的数据
        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();

        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        println!("{}", 
            str::from_utf8(&buffer).expect("Could not write buffer as string"));
        println!("");
    }

    Ok(())
}