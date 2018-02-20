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

import std.bitmanip : read, append, nativeToBigEndian;
import std.utf;
import std.socket;
import accessors;

/// Read an incoming byte
byte readUByte(ubyte[] buf) {
    return buf.read!ubyte;
}

/// Read an incoming signed byte
byte readByte(ubyte[] buf) {
    return buf.read!byte;
}

/// Read `size` number of bytes from the buffer
byte[] readBytes(ubyte[] buf, ulong size) {
    byte[] o = new byte[size];

    foreach (i; 0 .. size) {
        o ~= buf.readByte;
    }

    return o;
}

/// Read `size` number of bytes from the buffer
ubyte[] readUBytes(ubyte[] buf, ulong size) {
    ubyte[] o = new ubyte[size];

    foreach (i; 0 .. size) {
        o ~= buf.readUByte;
    }

    return o;
}

/// Read an incoming boolean
bool readBoolean(ubyte[] buf) {
    return buf.read!bool;
}

/// Read an incoming short
short readShort(ubyte[] buf) {
    return buf.read!short;
}

/// Read an incoming unsigned short
ushort readUShort(ubyte[] buf) {
    return buf.read!ushort;
}

/// Read an incoming integer
int readInt(ubyte[] buf) {
    return buf.read!int;
}

/// Read an incoming unsigned integer
uint readUInt(ubyte[] buf) {
    return buf.read!uint;
}

/// Read an incoming long
long readLong(ubyte[] buf) {
    return buf.read!long;
}

/// Read an incoming single-precision 32-bit floating point number
float readFloat(ubyte[] buf) {
    return buf.read!float;
}

/// Read an incoming double-precision 64-bit floating point number
double readDouble(ubyte[] buf) {
    return buf.read!double;
}

/// Read an incoming variable size integer
int readVarInt(ubyte[] buf) {
    int numRead;
    int result;
    byte read;

    do {
        read = buf.readByte;
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
long readVarLong(ubyte[] buf) {
    int numRead;
    long result;
    byte read;

    do {
        read = buf.readByte;
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
string readString(ubyte[] buf, int max = 32_767) {
    const int size = buf.readVarInt;

    if (size > (max * 4) + 3) {
        throw new Exception("String is too big");
    }
    const auto output = cast(string) buf.readBytes(size);
    validate(output);

    return output;
}

/// Write byte to outgoing buf
void writeByte(ubyte[] buf, const byte input) {
    buf.append!byte(input);
}

/// Write unsigned byte to outgoing buf
void writeUByte(ubyte[] buf, const ubyte input) {
    buf.append!ubyte(input);
}

/// Write bytes to the buffer
void writeBytes(ubyte[] buf, const byte[] input) {
    foreach (i; input) {
        buf.writeByte(i);
    }
}

/// Write bytes to the buffer
void writeUBytes(ubyte[] buf, const ubyte[] input) {
    foreach (i; input) {
        buf.writeUByte(i);
    }
}

/// Write boolean to outgoing buf
void writeBool(ubyte[] buf, const bool input) {
    buf.append!bool(input);
}

/// Write short to outgoing buf
void writeShort(ubyte[] buf, const short input) {
    buf.append!short(input);
}

/// Write unsigned short to outgoing buf
void writeUShort(ubyte[] buf, const ushort input) {
    buf.append!ushort(input);
}

/// Write int to outgoing buf
void writeInt(ubyte[] buf, const int input) {
    buf.append!int(input);
}

/// Write float to outgoing buf
void writeLong(ubyte[] buf, const long input) {
    buf.append!long(input);
}

/// Write float to outgoing buf
void writeFloat(ubyte[] buf, const float input) {
    buf.append!float(input);
}

/// Write double to outgoing buf
void writeDouble(ubyte[] buf, const double input) {
    buf.append!double(input);
}

/// Write varint to outgoing buf
void writeVarInt(ubyte[] buf, int value) {
    do {
        byte temp = cast(byte)(value & 0b01111111);

        value >>>= 7;
        if (value != 0) {
            temp |= 0b10000000;
        }
        buf.writeByte(temp);
    }
    while (value != 0);
}

/// Write string to outgoing buf
void writeString(ubyte[] buf, string input, int max = 32_767) {
    const ubyte[] bytes = cast(ubyte[]) input;
    const long size = bytes.length;

    if (size > (max * 4) + 3) {
        throw new Exception("String is too big");
    }

    buf.writeVarInt(cast(int) size);
    buf.writeUBytes(bytes);
}

/// Packet base
public abstract class Packet {
    private ubyte[] buffer;
    @Read @Write private int _id;

    /// Get a remote packet
    this(ubyte[] buffer) {
        this.buffer = buffer;
    }

    /// Create an empty packet
    this() {
        
    }

    /// Decode an incoming Packet
    public void decode();

    /// Encode an outgoing Packet
    public byte[] encode() nothrow;

    mixin(GenerateFieldAccessors);
}
