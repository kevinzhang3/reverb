use std::error::Error;

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};


const SERVER: Token = Token(0);

fn main() -> Result<(), Box<dyn Error>> {
    let mut poll = Poll::new()?;

    let mut events = Events::with_capacity(128);

    let addr = "127.0.0.1:8080".parse()?;
    let mut server = TcpListener::bind(addr)?;
    poll.registry().register(&mut server, SERVER, Interest::READABLE)?;

    loop {
        poll.poll(&mut events, None)?;

        // event processing
        for event in events.iter() {
            
            // match it based on the provided token 
            match event.token() {
                SERVER => {
                    let connection = server.accept()?;
                    drop(connection);
                }
                _ => unreachable!(),
            }
        }
    }
}
