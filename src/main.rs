use std::error::Error;
use std::io;

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};


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
                        },
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            println!("no events pending");
                        },
                        Err(e) => {
                            println!("error: {e:?}");
                        },
                    };
                    
                }
                _ => unreachable!(),
            }
        }
    }
}

// we want to give this function ownership of the stream, because after a stream
// is handled, we should drop it. (server should not keep a reference)
fn handler (mut stream: TcpStream) -> Result<&'static str, std::io::Error> {
    let buf_reader

    Ok("good")
}
