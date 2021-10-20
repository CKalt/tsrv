use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Activate echo mode
    #[structopt(short = "e", long = "echo")]
    echo: bool,
    #[structopt(short = "d", long = "ds_server")]
    ds_server: bool,
    #[structopt(short = "w", long = "web")]
    web: bool,
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
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = String::from_utf8_lossy(&buffer);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_ds_server_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);
    println!("received=[{}]", request);

    let response_header = r#"
{
  "holeNumber": "hole12",
  "archiveFilename": "Archive_211001_140321",
  "archiveTickCount": "845"
}
"#;

    let response_items = vec![r#"
{
   "confidence" : [
      0,
      0,
      0,
      0
   ],
   "digits" : [
      -1,
      -1,
      -1,
      -1
   ],
   "event_count" : 1,
   "events" : [
      "TEE_RECT"
   ],
   "label" : 0,
   "m_sphere.radius" : 2.43117189407349,        // radius of ball
   "pos" : [
      35.7651251552959,
      10.1044384407889,
      1.99999999999943
   ],
   "shot_count" : 0,
   "tick" : 104792,
   "time_sec" : 55675.611559,
   "vel" : [
      -1.43420169544014,
      -2.60537792789491,
      0
   ]
}
"#,r#"
{
   "confidence" : [
      0,
      0,
      0,
      0
   ],
   "digits" : [
      -1,
      -1,
      -1,
      -1
   ],
   "event_count" : 2,
   "events" : [
      "STOPPED"
   ],
   "label" : 0,
   "m_sphere.radius" : 1.81232150395711,
   "pos" : [
      35.4990601849867,
      10.256205154202,
      1.16103691994983
   ],
   "shot_count" : 0,
   "tick" : 104798,
   "time_sec" : 55676.211993,
   "vel" : [
      0.223588302871757,
      -0.378118526708058,
      0
   ]
}
"#,r#"
{
   "confidence" : [
      0,
      0,
      0,
      0
   ],
   "digits" : [
      -1,
      -1,
      -1,
      -1
   ],
   "event_count" : 3,
   "events" : [
      "MOVING"
   ],
   "label" : 0,
   "m_sphere.radius" : 1.75107336044312,
   "pos" : [
      441.823339168579,
      426.79626979401,
      -12
   ],
   "shot_count" : 1,
   "tick" : 105147,
   "time_sec" : 55711.134976,
   "vel" : [
      -7.20664948775637,
      -37.8342280718204,
      0
   ]
}
"#,r#"
{
   "confidence" : [
      0,
      0,
      0,
      0
   ],
   "digits" : [
      -1,
      -1,
      -1,
      -1
   ],
   "event_count" : 4,
   "events" : [
      "CUP"
   ],
   "label" : 0,
   "m_sphere.radius" : 1.74538373947144,
   "pos" : [
      439.592231994182,
      429.815724232712,
      -12
   ],
   "shot_count" : 1,
   "tick" : 105157,
   "time_sec" : 55712.134988,
   "vel" : [
      0.119569116498076,
      0.153994787065098,
      0
   ]
}
"#];

    let response_footer = r#"
{
  "holeNumber": "hole12",
  "archiveFilename": "Archive_211001_140321",
  "Results": "DONE"
}
"#;

    println!("sending response_header=[{}]", response_header);
    stream.write(response_header.as_bytes()).unwrap();
    for response_item in response_items.iter() {
        println!("sending response_item=[{}]", response_item);
        stream.write(response_item.as_bytes()).unwrap();
    }
    println!("sending response_footer=[{}]", response_footer);
    stream.write(response_footer.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let listener = TcpListener::bind("localhost:8080").unwrap();

    let handle_connection: Handler = 
        if opt.echo { 
            println!("using echo");
            handle_echo_connection
        } else if opt.ds_server {
            println!("using web)");
            handle_web_connection 
        } else {
            println!("using ds_server");
            handle_ds_server_connection 
        };

    for stream in listener.incoming() {
        let data = stream.unwrap();

        println!("Connection established!");
        handle_connection(data);
    }
}
