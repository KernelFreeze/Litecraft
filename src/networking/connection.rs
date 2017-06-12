use std::net::TcpStream;
use std::time::Duration;

#[derive(PartialEq, Eq, Debug)]
enum ConnectionErr {
    ConnectFailed,
}

#[derive(PartialEq, Eq, Debug)]
enum ConnectionState {
    Handshaking,
    Play,
    Status,
    Login
}

pub struct Connection {
    stream: TcpStream,
    state: ConnectionState
}

impl Connection {
    /// Connect to a server
    fn new(url: &str) -> Result<Connection, ConnectionErr> {
        if let Ok(stream) = TcpStream::connect(url) {
            stream.set_read_timeout(Some(Duration::new(10, 0)));
            stream.set_write_timeout(Some(Duration::new(10, 0)));

            let state = ConnectionState::Handshaking;
            return Ok(Connection { stream, state });
        }
        Err(ConnectionErr::ConnectFailed)
    }
}