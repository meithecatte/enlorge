use anyhow::{anyhow, Context, Result};
use std::fs;

mod deflate;
mod gzip;
mod helper;
mod huffman;

fn main() -> Result<()> {
    let filename = std::env::args().nth(1)
        .ok_or_else(|| anyhow!("usage: enlorge <filename> [<outfile>]"))?;
    let data = fs::read(filename).context("while reading file")?;

    let mut buf: &[u8] = &data;
    let output = gzip::decompress(&mut buf)?;

    if let Some(outname) = std::env::args().nth(2) {
        fs::write(outname, output).context("while writing file")?;
    }

    Ok(())
}
