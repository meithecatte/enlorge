use anyhow::{anyhow, Context, Result};
use bytes::BytesMut;
use std::fs;

mod deflate;
mod gzip;
mod helper;

fn main() -> Result<()> {
    let filename = std::env::args().nth(1)
        .ok_or_else(|| anyhow!("usage: enlorge <filename>"))?;
    let data = fs::read(filename).context("while reading file")?;

    let mut buf: &[u8] = &data;
    let header = gzip::read_header(&mut buf)?;
    dbg!(header);

    let mut output = BytesMut::new();
    deflate::decompress(&mut buf, &mut output)?;
    dbg!(output);
    Ok(())
}
