use uuid::Uuid;
use std::io;
use std::io::Read;
use std::io::Cursor;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};

trait Packet {
    /// Create a new empty packet
    fn new(id: i32, buffer: Vec<u8>) -> Self;

    /// Get packet buffer
    fn buffer(&self) -> Cursor<Vec<u8>>;

    /// Read bytes
    fn read(&mut self, len: usize) -> Vec<u8> {
        let mut out = vec![0; len];
        self.buffer().read(&mut out);
        out
    }

    /// Read boolean
    fn read_bool(&mut self) -> bool {
        self.read(1)[0] == 0x1u8
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

    // Decode a byte array into a packet
    // pub fn decode(data: &[u8]) -> Option<Packet> {}

    // Encode the packet into a byte array
    // pub fn encode(&self) -> &[u8] {}
}
