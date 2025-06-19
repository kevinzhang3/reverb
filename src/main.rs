use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() -> std::io::Result<()> {
    
    // ? is equivalent to:
    // match x {
    //   Ok(v) => v,
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
    let request_line = match buf_reader.lines().next() {
        Some(line) => line,
        None => panic!("ERROR: failed to parse http request"),
    }?;

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "rust.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes())?;
    
    Ok(())
}
