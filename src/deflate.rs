use anyhow::{bail, Result};
use bytes::{Buf, BufMut};
use crate::helper::BitReader;
use crate::huffman::{BitStream, Huffman};
use std::iter;

pub fn decompress(input: &mut impl Buf) -> Result<Vec<u8>> {
    let mut input = BitReader::new(input);
    let mut output = Vec::new();
    let fixed = BlockContext::fixed();
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
            0b01 => {
                fixed.decompress(&mut input, &mut output)?;
            }
            0b10 => todo!("dynamic Huffman code"),
            _ => bail!("deflate: unknown block type"),
        }

        if bfinal {
            break;
        }
    }

    Ok(output)
}

#[derive(Clone)]
struct BlockContext {
    main: Huffman,
    dist: Huffman,
}

impl BlockContext {
    fn fixed() -> Self {
        let lengths: Vec<u8> = iter::repeat(8).take(144)
            .chain(iter::repeat(9).take(256 - 144))
            .chain(iter::repeat(7).take(280 - 256))
            .chain(iter::repeat(8).take(288 - 280))
            .collect();
        Self {
            main: Huffman::new(&lengths),
            dist: Huffman::new(&[5; 32]),
        }
    }

    fn decompress<B: Buf>(
        &self,
        input: &mut BitReader<B>,
        output: &mut Vec<u8>,
    ) -> Result<()> {
        loop {
            match self.main.decode(input) {
                c @ 0..256 => {
                    output.push(c as u8);
                }
                256 => return Ok(()),
                c @ 257..=285 => {
                    let len = self.get_length(input, c - 257);
                    let dist = self.get_dist(input)? as usize;
                    for _ in 0..len {
                        output.push(output[output.len() - dist]);
                    }
                }
                _ => bail!("invalid literal/length code"),
            }
        }
    }

    fn get_length<B: Buf>(
        &self,
        input: &mut BitReader<B>,
        c: u16,
    ) -> u16 {
        if c < 8 {
            c + 3
        } else if c == 28 {
            258
        } else {
            let extra = (c / 4 - 1) as u8;
            let len = input.get_bits(extra) as u16 | (c % 4 + 4) << extra;
            len + 3
        }
    }

    fn get_dist<B: Buf>(&self, input: &mut BitReader<B>) -> Result<u16> {
        let c = self.dist.decode(input);
        if c < 4 {
            Ok(c + 1)
        } else if c >= 30 {
            bail!("invalid distance code")
        } else {
            let extra = (c / 2 - 1) as u8;
            let dist = input.get_bits(extra) as u16 | (c % 2 + 2) << extra;
            Ok(dist + 1)
        }
    }
}
