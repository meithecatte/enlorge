use anyhow::{bail, Result};
use bytes::{Buf, BufMut};
use crate::helper::BitReader;

pub fn decompress(input: &mut impl Buf, output: &mut impl BufMut) -> Result<()> {
    let mut input = BitReader::new(input);
    loop {
        let bfinal = input.get_bit();
        match input.get_bits(2) {
            // no compression
            0b00 => {
                let buf = input.drop_align();
                let len = buf.get_u16_le();
                let nlen = buf.get_u16_le();
                if len != !nlen {
                    bail!("deflate: corrupted block length");
                }

                output.put(buf.take(len.into()));
            }
            0b01 => todo!("fixed Huffman code"),
            0b10 => todo!("dynamic Huffman code"),
            _ => bail!("deflate: unknown block type"),
        }

        if bfinal {
            break;
        }
    }

    Ok(())
}
