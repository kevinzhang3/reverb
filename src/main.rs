use std::{
    fs,
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
        
        handle_connection(stream)?;
    }
    Ok(())
}

// handler takes the connection stream and wraps it in a buffer
// the buffered data is read line by line and collected into a vector using .collect()
fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| match result {
            Ok(v) => v,
            Err(e) => panic!("ERROR: failed to read line {e}"),
        }) 
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("rust.html")?;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    println!("HTTP Response: {http_request:#?}");
    stream.write_all(response.as_bytes())?; 
    
    Ok(())
}
