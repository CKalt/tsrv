use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Activate echo mode
    #[structopt(short = "e", long = "echo")]
    echo: bool,
}

type Handler = fn(TcpStream);

fn handle_web_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_echo_connection(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let handle_connection: Handler = 
        if opt.echo { println!("using echo");  handle_echo_connection } 
        else { println!("using web)"); handle_web_connection };

    for stream in listener.incoming() {
        let data = stream.unwrap();

        println!("Connection established!");
        handle_connection(data);
    }
}

