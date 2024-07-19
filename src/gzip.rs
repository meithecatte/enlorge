use anyhow::{anyhow, bail, Result};
use bytes::{Buf, Bytes};
use crate::helper::BufExt;
use num_enum::TryFromPrimitive;
use bitflags::bitflags;
use std::ffi::CString;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct GzipHeader {
    pub flags: Flags,
    pub mtime: u32,
    pub extra_flags: u8,
    pub os: OS,
    pub extra: Option<Bytes>,
    pub filename: Option<CString>,
    pub comment: Option<CString>,
}

#[derive(Clone, Copy, Debug, PartialEq, TryFromPrimitive)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum OS {
    FAT = 0,
    Amiga = 1,
    VMS = 2,
    Unix = 3,
    VM_CMS = 4,
    AtariTOS = 5,
    HPFS = 6,
    Macintosh = 7,
    ZSystem = 8,
    CPM = 9,
    TOPS20 = 10,
    NTFS = 11,
    QDOS = 12,
    RISCOS = 13,
    Unknown = 255,
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Flags: u8 {
        const FTEXT = 0x01;
        const FHCRC = 0x02;
        const FEXTRA = 0x04;
        const FNAME = 0x08;
        const FCOMMENT = 0x10;
    }
}

pub fn read_header(buf: &mut impl Buf) -> Result<GzipHeader> {
    if buf.get_u16_le() != 0x8b1f {
        bail!("gzip: invalid magic number");
    }

    if buf.get_u8() != 8 {
        bail!("gzip: unknown compression method");
    }

    // A compliant decompressor must give an error indication if any reserved
    // bit is non-zero, since such a bit could indicate the presence of a new
    // field that would cause subsequent data to be interpreted incorrectly.
    //
    // ~ RFC1952, 2.3.1.2. Compliance
    let flags = Flags::from_bits(buf.get_u8())
        .ok_or_else(|| anyhow!("gzip: unknown header flags"))?;

    let mtime = buf.get_u32_le();
    let extra_flags = buf.get_u8();
    let os = buf.get_u8().try_into().unwrap_or(OS::Unknown);

    let extra = if flags.contains(Flags::FEXTRA) {
        let xlen = buf.get_u16_le();
        Some(buf.copy_to_bytes(xlen.into()))
    } else {
        None
    };

    let filename = if flags.contains(Flags::FNAME) {
        Some(buf.get_cstring())
    } else {
        None
    };

    let comment = if flags.contains(Flags::FCOMMENT) {
        Some(buf.get_cstring())
    } else {
        None
    };

    if flags.contains(Flags::FHCRC) {
        // [A compliant decompressor] must examine [...] FHCRC at least so it
        // can skip over the optional fields if they are present.
        //
        // TODO: actually check this
        let _ = buf.get_u16_le();
    }

    Ok(GzipHeader {
        flags,
        mtime,
        extra_flags,
        os,
        extra,
        filename,
        comment,
    })
}
