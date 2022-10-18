use std::error::Error;
use std::fmt::{self, Display};

#[cfg(test)]
mod tests;

const ENCODE_TABLE: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const DECODE_TABLE: [u8; 256] = {
    let mut decode_map = [0xFF; 256];
    let mut i = 0;
    while i < 62 {
        decode_map[ENCODE_TABLE[i] as usize] = i as u8;
        i += 1;
    }
    decode_map
};
const COMPACT_MASK: u8 = 0b00011110;
const MASK_5_BITS: u8 = 0b00011111;
const MASK_6_BITS: u8 = 0b00111111;

fn encode_symbol(six_bits: u8) -> (char, u8) {
    let (i, n) = if (six_bits & COMPACT_MASK) == COMPACT_MASK {
        (six_bits & MASK_5_BITS, 5)
    } else {
        (six_bits, 6)
    };
    (char::from(ENCODE_TABLE[i as usize]), n)
}

pub fn encode(src: &[u8]) -> String {
    let mut out = String::new();

    if !src.is_empty() {
        // Reserve space for `ceil(src.len() * 8 / 5)` bytes (in case we emit exclusively five bit
        // symbols).
        out = String::with_capacity((src.len() * 8 - 1) / 5 + 1);

        // Maintain a buffer of at most 5 trailing bits from the previous byte.
        let mut num_trailing = 0u8;
        let mut trailing_bits = 0u8;

        for &byte in src {
            let mut bits_left = 8 + num_trailing;

            // Read six bits and emit a symbol.
            let first_six = (trailing_bits | byte << num_trailing) & MASK_6_BITS;
            let (symbol, n) = encode_symbol(first_six);

            out.push(symbol);
            bits_left -= n;

            // If we have enough bits left, emit a second symbol.
            if bits_left >= 6 {
                let second_six = (byte >> (8 - bits_left)) & MASK_6_BITS;
                let (symbol, n) = encode_symbol(second_six);

                out.push(symbol);
                bits_left -= n;
            }

            // Stash our remaining bits for the next byte.
            num_trailing = bits_left;
            trailing_bits = if num_trailing > 0 {
                byte >> (8 - bits_left)
            } else {
                0
            };
        }
        if num_trailing > 0 {
            let (symbol, _) = encode_symbol(trailing_bits);
            out.push(symbol);
        }
    }

    out
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DecodeError {
    InvalidByte(usize, u8),
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let DecodeError::InvalidByte(index, symbol) = self;
        write!(f, "Invalid byte '{symbol}' at index {index}")
    }
}

impl Error for DecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn decode_symbol(index: usize, symbol: u8) -> Result<(u8, u8), DecodeError> {
    let bits = DECODE_TABLE[symbol as usize];
    if bits == 0xFF {
        return Err(DecodeError::InvalidByte(index, symbol));
    }
    let num_bits = if bits & COMPACT_MASK == COMPACT_MASK {
        5
    } else {
        6
    };
    Ok((bits, num_bits))
}

pub fn decode(src: &str) -> Result<Vec<u8>, DecodeError> {
    let mut out = vec![];
    if let Some((&last, bytes)) = src.as_bytes().split_last() {
        // Reserve space for `ceil(src.len() * 6 / 8) bytes` (in case every input symbol is six bits).
        out = Vec::with_capacity((src.len() * 6 - 1) / 8 + 1);

        // Maintain a buffer of at most 7 bits left over from the previous input symbol.
        let mut buf = 0u8;
        let mut buf_size = 0;

        for (i, &symbol) in bytes.iter().enumerate() {
            let (bits, num_bits) = decode_symbol(i, symbol)?;

            // Emit an input byte if we've filled a full byte...
            if buf_size + num_bits >= 8 {
                out.push((bits << buf_size) | buf);
                buf = bits >> (8 - buf_size);
                buf_size = num_bits - (8 - buf_size);
            }
            // ...or queue up our bits to be processed next round.
            else {
                buf |= bits << buf_size;
                buf_size += num_bits;
            }
        }
        let (bits, _) = decode_symbol(src.len() - 1, last)?;
        out.push(bits << buf_size | buf);
    }
    Ok(out)
}
