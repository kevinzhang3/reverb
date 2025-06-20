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
        match router(stream) {
            Ok(v) => println!("Handled request: {:#?}", v),
            Err(e) => println!("ERROR: Bad request: {e}"),
        }
    }
    Ok(())
}

// handler takes the connection stream and wraps it in a buffer
// the buffered data is read line by line and collected into a vector using .collect()
fn router(mut stream: TcpStream) -> Result<&'static str, std::io::Error> {
    let buf_reader = BufReader::new(&stream);
    
    let request = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => return Err(e.into()), // propagate the I/O error
        None => return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "No input received").into()),
    };

    let (status, filename) = match request.split_whitespace().nth(1).unwrap_or("/") {
        "/" => ("HTTP/1.1 200 OK", "rust.html"), // replace these with functions based on URI
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();
    let response = format!(
        "{status}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes())?;

    Ok(status)
}

