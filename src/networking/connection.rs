use std::net::TcpStream;
use std::time::Duration;
use std::io::Read;
use std::io::Cursor;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};

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

    /// Read bytes
    fn read(&mut self, len: usize) -> Vec<u8> {
        let mut buffer = vec![0; len];
        self.stream.read(&mut buffer[..]);
        buffer
    }

    /// Read boolean
    fn read_bool(&mut self) -> bool {
        let mut buffer = [0, 1];
        self.stream.read(&mut buffer[..]);
        buffer[0] == 0x1u8
    }

    /// Read single signed byte
    fn read_byte(&mut self) -> i8 {
        let mut rdr = Cursor::new(self.read(1));
        rdr.read_i8().unwrap()
    }

    /// Read single unsigned byte
    fn read_ubyte(&mut self) -> u8 {
        let mut rdr = Cursor::new(self.read(1));
        rdr.read_u8().unwrap()
    }

    /// Read single signed short
    fn read_short(&mut self) -> i16 {
        let mut rdr = Cursor::new(self.read(2));
        rdr.read_i16::<BigEndian>().unwrap()
    }

    /// Read single unsigned short
    fn read_ushort(&mut self) -> u16 {
        let mut rdr = Cursor::new(self.read(2));
        rdr.read_u16::<BigEndian>().unwrap()
    }

    /// Read single signed int
    fn read_int(&mut self) -> i32 {
        let mut rdr = Cursor::new(self.read(4));
        rdr.read_i32::<BigEndian>().unwrap()
    }

    /// Read single unsigned int
    fn read_uint(&mut self) -> u32 {
        let mut rdr = Cursor::new(self.read(4));
        rdr.read_u32::<BigEndian>().unwrap()
    }

    /// Read single signed long
    fn read_long(&mut self) -> i64 {
        let mut rdr = Cursor::new(self.read(8));
        rdr.read_i64::<BigEndian>().unwrap()
    }

    /// Read single unsigned long
    fn read_ulong(&mut self) -> u64 {
        let mut rdr = Cursor::new(self.read(8));
        rdr.read_u64::<BigEndian>().unwrap()
    }

    /// Read single signed float
    fn read_float(&mut self) -> f32 {
        let mut rdr = Cursor::new(self.read(4));
        rdr.read_f32::<BigEndian>().unwrap()
    }

    /// Read single signed float
    fn read_double(&mut self) -> f64 {
        let mut rdr = Cursor::new(self.read(8));
        rdr.read_f64::<BigEndian>().unwrap()
    }
}