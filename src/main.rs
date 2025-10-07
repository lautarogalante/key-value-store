use std::{env::args, io::{self, Read, Write}, net::{TcpListener, TcpStream}};
use kv_store::kv;


fn handle_connection(stream: &mut TcpStream) ->std::io::Result<i32> {
    let mut buffer = [0; 1024];
    let mut in_memory = kv::Store::new();
    
    loop  {
        let n = match stream.read(&mut buffer) {
            Ok(0) => {
                let _ = io::stdout().write_all(b"Close connection\n");
                return Ok(0);
            },
            Ok(n) => n,
            Err(e) => {
                println!("Error reading from buffer: {e}");
                return Err(e);
            }
        };
        
        let input = String::from_utf8(buffer[..n].to_vec());
        let value = in_memory.execute(&input.unwrap());
        let _ = stream.write_all(format!("{:?}\n", value).as_bytes());
    }
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();
    let mut port: u16 = 8080;
    if args.len() > 1 {
        match args[1].parse::<u16>() {
            Ok(argument ) => {
                port = argument
            }
            Err(e) => {
                println!("The argument must be an integer: {e}");
                return Ok(())
            }
        }
    }

    let listener = TcpListener::bind(("127.0.0.1", port))?;
    let _ = io::stdout().write_all(format!("Welcome to KV-Store - Listening on port: {}\n", port).as_bytes());
    
    let mut status: i32;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                status = handle_connection(&mut stream)?;
                if status == 0 {
                    break;
                }
            }
            Err(e) => { println!("{e}")}
        }
    }
    Ok(())
}
