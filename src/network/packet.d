/*
 * Copyright 2014-2018 Miguel Peláez <kernelfreeze@outlook.com>
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software
 * and associated documentation files (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge, publish, distribute,
 * sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all copies or
 * substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
 * BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
module network.packet;

import std.bitmanip : peek, nativeToBigEndian;
import std.utf;
import std.socket;

/// Read a defined amount of bytes
ubyte[] readUBytes(TcpSocket connection, int amount = 1) {
    ubyte[] o = new ubyte[amount];
    connection.receive(o);

    return o;
}

/// Read incoming signed bytes
byte[] readBytes(TcpSocket connection, int amount = 1) {
    return cast(byte[]) connection.readUBytes(amount);
}

/// Read an incoming byte
byte readUByte(TcpSocket connection) {
    return connection.readUBytes[0];
}

/// Read an incoming signed byte
byte readByte(TcpSocket connection) {
    return connection.readBytes[0];
}

/// Read an incoming boolean
bool readBoolean(TcpSocket connection) {
    return connection.readUBytes.peek!bool;
}

/// Read an incoming short
short readShort(TcpSocket connection) {
    return connection.readUBytes(2).peek!short;
}

/// Read an incoming unsigned short
ushort readUShort(TcpSocket connection) {
    return connection.readUBytes(2).peek!ushort;
}

/// Read an incoming integer
int readInt(TcpSocket connection) {
    return connection.readUBytes(4).peek!int;
}

/// Read an incoming unsigned integer
uint readUInt(TcpSocket connection) {
    return connection.readUBytes(4).peek!uint;
}

/// Read an incoming long
long readLong(TcpSocket connection) {
    return connection.readUBytes(8).peek!long;
}

/// Read an incoming single-precision 32-bit floating point number
float readFloat(TcpSocket connection) {
    return connection.readUBytes(4).peek!float;
}

/// Read an incoming double-precision 64-bit floating point number
double readDouble(TcpSocket connection) {
    return connection.readUBytes(8).peek!double;
}

/// Read an incoming variable size integer
int readVarInt(TcpSocket connection) {
    int numRead;
    int result;
    byte read;

    do {
        read = connection.readByte;
        const int value = (read & 0b01111111);
        result |= (value << (7 * numRead));

        numRead++;
        if (numRead > 5) {
            throw new Exception("VarInt is too big");
        }
    }
    while ((read & 0b10000000) != 0);

    return result;
}

/// Read an incoming variable size integer
long readVarLong(TcpSocket connection) {
    int numRead;
    long result;
    byte read;

    do {
        read = connection.readByte;
        const int value = (read & 0b01111111);
        result |= (value << (7 * numRead));

        numRead++;
        if (numRead > 10) {
            throw new Exception("VarLong is too big");
        }
    }
    while ((read & 0b10000000) != 0);

    return result;
}

/// Read a incoming string
string readString(TcpSocket connection, int max = 32_767) {
    const int size = connection.readVarInt;

    if (size > (max * 4) + 3) {
        throw new Exception("String is too big");
    }
    const auto output = cast(string) connection.readBytes(size);
    validate(output);

    return output;
}

/// Write byte to outgoing connection
void writeByte(TcpSocket connection, byte b) {
    ubyte[1] o = [cast(ubyte) b];
    connection.send(o);
}

/// Write unsigned byte to outgoing connection
void writeUByte(TcpSocket connection, ubyte b) {
    ubyte[1] o = [b];
    connection.send(o);
}

/// Write boolean to outgoing connection
void writeBool(TcpSocket connection, bool input) {
    connection.send(nativeToBigEndian(input));
}

/// Write short to outgoing connection
void writeShort(TcpSocket connection, short input) {
    connection.send(nativeToBigEndian(input));
}

/// Write unsigned short to outgoing connection
void writeUShort(TcpSocket connection, ushort input) {
    connection.send(nativeToBigEndian(input));
}

/// Write int to outgoing connection
void writeInt(TcpSocket connection, int input) {
    connection.send(nativeToBigEndian(input));
}

/// Write float to outgoing connection
void writeLong(TcpSocket connection, long input) {
    connection.send(nativeToBigEndian(input));
}

/// Write float to outgoing connection
void writeFloat(TcpSocket connection, float input) {
    connection.send(nativeToBigEndian(input));
}

/// Write double to outgoing connection
void writeDouble(TcpSocket connection, double input) {
    connection.send(nativeToBigEndian(input));
}

/// Write varint to outgoing connection
void writeVarInt(TcpSocket connection, int value) {
    do {
        byte temp = cast(byte)(value & 0b01111111);

        value >>>= 7;
        if (value != 0) {
            temp |= 0b10000000;
        }
        connection.writeByte(temp);
    }
    while (value != 0);
}

/// Write string to outgoing connection
void writeString(TcpSocket connection, string input, int max = 32_767) {
    const ubyte[] bytes = cast(ubyte[]) input;
    const long size = bytes.length;

    if (size > (max * 4) + 3) {
        throw new Exception("String is too big");
    }

    connection.writeVarInt(cast(int) size);
    connection.send(bytes);
}

/// Packet base
public abstract class Packet {
    private TcpSocket connection;

    /// Create a new Packet using an open connection
    this(TcpSocket connection) {
        this.connection = connection;
    }

    /// Decode an incoming Packet
    public void decode();

    /// Encode an outgoing Packet, you must write your packet ID to the byte stream
    public void encode() nothrow;
}
