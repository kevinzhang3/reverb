use std::{
    error::Error,
    fs,
    io::{BufReader, prelude::*},
};
use mio::{
    net::{TcpListener, TcpStream},
    {Events, Interest, Poll, Token},
};


const SERVER: Token = Token(0);

fn main() -> Result<(), Box<dyn Error>> {
    let mut poll = Poll::new()?;

    let mut events = Events::with_capacity(128);

    let addr = "127.0.0.1:8080".parse()?;
    let mut server = TcpListener::bind(addr)?;
    poll.registry().register(&mut server, SERVER, Interest::READABLE)?;

    println!("Server started.");
    loop {
        poll.poll(&mut events, None)?;

        // event processing
        for event in events.iter() {
            
            // match it based on the provided token 
            match event.token() {
                SERVER => {
                    let connection = match server.accept() {
                        Ok(addr) => {
                            println!("new client: {addr:?}");
                            Some(addr)
                        },
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            println!("no events pending");
                            None
                        },
                        Err(e) => {
                            println!("error: {e:?}");
                            None
                        },
                    };
                    // this line is kind of gross but basically server.accept()
                    // returns a tuple with a (tcpstream, addr)
                    handler(connection.expect("stream failed").0)?;
                }
                _ => unreachable!(),
            }
        }
    }
}

// we want to give this function ownership of the stream, because after a stream
// is handled, we should drop it. (server should not keep a reference)
fn handler (mut stream: TcpStream) -> Result<&'static str, std::io::Error> {
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

    Ok("status")
}
