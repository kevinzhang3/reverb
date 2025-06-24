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

    Ok(())
}
