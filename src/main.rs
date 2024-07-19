#[macro_use] extern crate enum_primitive;

use anyhow::{anyhow, Context, Result};
use std::fs;

mod gzip;
mod helper;

fn main() -> Result<()> {
    let filename = std::env::args().nth(1)
        .ok_or_else(|| anyhow!("usage: enlorge <filename>"))?;
    let data = fs::read(filename).context("while reading file")?;
    let header = gzip::read_header(&mut &*data)?;
    dbg!(header);
    Ok(())
}
