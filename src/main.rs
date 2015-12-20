use std::string::String;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
//use std::thread;


fn handle_client(mut stream: TcpStream) {

    loop {
        let mut buffer : Vec<u8> = vec![0; 1024];
        let len = stream.read(&mut buffer[..]).unwrap();
        print!("{}", String::from_utf8(buffer.clone()).unwrap());

        //let bytes = buffer.clone();
        let _ = stream.write_all(&buffer);

        if len == 0 {
            let _ = stream.shutdown(Shutdown::Both);
            break;
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => { 
                print!("Connection failed: {}", e);
            }
        }
    }

    drop(listener);
}
