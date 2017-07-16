use futures::Future;
use std::net::SocketAddr;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;

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
    state: ConnectionState
}

impl Connection {
    /// Connect to a server
    fn new(addr: &str) -> Result<Connection, ConnectionErr> {
        let addr = addr.parse::<SocketAddr>().unwrap();

        // Create the event loop and initiate the connection to the remote server
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let tcp = TcpStream::connect(&addr, &handle);

        let client = tcp.and_then(|stream| {
            Ok(())
        });

        match core.run(client) {
            Err(_) => return Err(ConnectionErr::ConnectFailed),
            Ok(_) => {
                let state = ConnectionState::Handshaking;
                return Ok(Connection { state });
            }
        }
    }
}