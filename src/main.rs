use std::error::Error;

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};


const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() -> Result<(), Box<dyn Error>> {
    let mut poll = Poll::new()?;

    let mut events = Events::with_capacity(128);

    let addr = "127.0.0.1:8080".parse()?;
    let mut server = TcpListener::bind(addr)?;
    poll.registry().register(&mut server, SERVER, Interest::READABLE)?;

    let mut client = TcpStream::connect(addr)?;
    poll.registry().register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;

    loop {
        poll.poll(&mut events, None)?;

        // event processing
        for event in events.iter() {
            
            // match it based on the provided token 
            match event.token() {
                SERVER => {
                    let connection = server.accept();
                    drop(connection);
                }
                CLIENT => {
                    if event.is_writable() {
                        // write stuff
                    }

                    if event.is_readable() {
                        // read stuff 
                    }

                    return Ok(());
                }
                _ => unreachable!(),
            }
        }
    }
}
