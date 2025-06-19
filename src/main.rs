use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() -> std::io::Result<()> {
    
    // ? is equivalent to:
    // match x {
    //   Ok(v) => v
    //   Err(e) => return Err(e.into()),
    // }
    // where x returns Result<(v, e)>
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    // .incoming() returns an Incoming struct which is an iterator infinitely
    // accepts connections on TcpListener.
    //
    // it iterates over streams, each stream corresponds to one connection,
    // which is passed to the handler 
    for stream in listener.incoming() {
        let stream = stream?;
        
        handle_connection(stream);
    }
    Ok(())
}

// handler takes the connection stream and wraps it in a buffer
// the buffered data is read line by line and collected into a vector using .collect()
fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap()) // should try not to panic here, ideally handle gracefully
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
