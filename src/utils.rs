use std::io::{Cursor, Error, ErrorKind, Read, Result};
use std::str::FromStr;

/// Converts a string like "0x65" or "101" to u32
pub fn parse_packet_code(s: &str) -> Result<u32> {
    if s.starts_with("0x") || s.starts_with("0X") {
        u32::from_str_radix(&s[2..], 16)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e))
    } else {
        u32::from_str(s)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e))
    }
}

/// Reads a 4-byte integer (big-endian) from the cursor.
pub fn read_int(cursor: &mut Cursor<&[u8]>) -> Result<i32> {
    let mut buf = [0u8; 4];
    cursor.read_exact(&mut buf)?;
    Ok(
        ((buf[0] as i32) << 24) |
            ((buf[1] as i32) << 16) |
            ((buf[2] as i32) << 8)  |
            (buf[3] as i32)
    )
}

/// Writes a 4-byte integer (big-endian) to a vector.
pub fn write_int(buf: &mut Vec<u8>, value: i32) -> Result<()> {
    buf.extend(&[
        ((value >> 24) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 8)  & 0xFF) as u8,
        (value         & 0xFF) as u8,
    ]);
    Ok(())
}

/// Reads a VarInt from a slice and returns (value, bytes_read).
pub fn read_varint(data: &[u8]) -> Result<(u32, usize)> {
    let mut num_read = 0;
    let mut result = 0u32;
    for byte in data {
        let value = (byte & 0x7F) as u32;
        result |= value << (7 * num_read);
        num_read += 1;
        if byte & 0x80 == 0 {
            return Ok((result, num_read));
        }
        if num_read > 5 {
            return Err(Error::new(ErrorKind::InvalidData, "VarInt too long"));
        }
    }
    Err(Error::new(ErrorKind::UnexpectedEof, "Could not read complete VarInt"))
}
