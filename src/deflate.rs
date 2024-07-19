use anyhow::{bail, Result};
use bytes::{Buf, BufMut};
use crate::helper::BitReader;

pub fn decompress(input: &mut impl Buf, output: &mut impl BufMut) -> Result<()> {
    let mut input = BitReader::new(input);
    loop {
        let bfinal = input.get_bit();
        match input.get_bits(2) {
            0b00 => todo!("no compression"),
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
