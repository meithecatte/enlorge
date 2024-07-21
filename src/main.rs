use anyhow::{anyhow, Context, Result};
use std::fs;

mod deflate;
mod gzip;
mod helper;
mod huffman;

fn main() -> Result<()> {
    let filename = std::env::args().nth(1)
        .ok_or_else(|| anyhow!("usage: enlorge <filename>"))?;
    let data = fs::read(filename).context("while reading file")?;

    let mut buf: &[u8] = &data;
    let output = gzip::decompress(&mut buf)?;
    dbg!(output);
    Ok(())
}
