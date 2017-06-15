use uuid::Uuid;
use std::io;
use std::io::Read;
use std::io::Cursor;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};

#[derive(PartialEq, Eq, Debug)]
enum ReadErr {
    VarIntTooLong,
}

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
        self.buffer().read_u8().unwrap() == 0x1u8
    }

    /// Read single signed byte
    fn read_byte(&mut self) -> i8 {
        self.buffer().read_i8().unwrap()
    }

    /// Read single unsigned byte
    fn read_ubyte(&mut self) -> u8 {
        self.buffer().read_u8().unwrap()
    }

    /// Read single signed short
    fn read_short(&mut self) -> i16 {
        self.buffer().read_i16::<BigEndian>().unwrap()
    }

    /// Read single unsigned short
    fn read_ushort(&mut self) -> u16 {
        self.buffer().read_u16::<BigEndian>().unwrap()
    }

    /// Read single signed int
    fn read_int(&mut self) -> i32 {
        self.buffer().read_i32::<BigEndian>().unwrap()
    }

    /// Read single unsigned int
    fn read_uint(&mut self) -> u32 {
        self.buffer().read_u32::<BigEndian>().unwrap()
    }

    /// Read single signed long
    fn read_long(&mut self) -> i64 {
        self.buffer().read_i64::<BigEndian>().unwrap()
    }

    /// Read single unsigned long
    fn read_ulong(&mut self) -> u64 {
        self.buffer().read_u64::<BigEndian>().unwrap()
    }

    /// Read single signed float
    fn read_float(&mut self) -> f32 {
        self.buffer().read_f32::<BigEndian>().unwrap()
    }

    /// Read single signed float
    fn read_double(&mut self) -> f64 {
        self.buffer().read_f64::<BigEndian>().unwrap()
    }

    /// Read VarInt
    fn read_varint(&mut self) -> Result<i32, ReadErr> {
        let mut size = 0;
        let mut val = 0u32;

        loop {
            let b = self.buffer().read_u8().unwrap() as u32;
            val |= (b & 0x7F) << (size * 7);
            size += 1;

            if size > 5 {
                return Err(ReadErr::VarIntTooLong);
            } else if (b & 0x80) == 0 {
                break
            }
        }

        Result::Ok(val as i32)
    }

    /// Read VarLong
    fn read_varlong(&mut self) -> Result<i64, ReadErr> {
        let mut size = 0;
        let mut val = 0u64;

        loop {
            let b = self.buffer().read_u8().unwrap() as u64;
            val |= (b & 0x7F) << (size * 7);
            size += 1;

            if size > 10 {
                return Err(ReadErr::VarIntTooLong);
            } else if (b & 0x80) == 0 {
                break
            }
        }

        Result::Ok(val as i64)
    }

    /// Decode a byte array into a packet
    fn decode(&mut self);

    /// Encode the packet into a byte array
    fn encode(&self) -> &[u8];
}
