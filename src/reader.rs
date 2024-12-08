mod dict;
pub mod error;

use crate::consts::{
    HEADER_BLOCK_SIZE, MAGIC_STRING, SIZE_HEADER_LEN, SIZE_MAJOR_VERSION, SIZE_MINOR_VERSION,
};
use crate::error::ReadHeaderError;
use std::io::Read;

fn read_exact(f: &mut std::fs::File, buf: &mut [u8]) -> Result<(), ReadHeaderError> {
    f.read_exact(buf).map_err(ReadHeaderError::Io)
}

pub fn check_magic_string(f: &mut std::fs::File) -> Result<(), ReadHeaderError> {
    let mut buf = [0u8; MAGIC_STRING.len()];
    read_exact(f, &mut buf)?;
    if MAGIC_STRING != buf {
        return Err(ReadHeaderError::InvalidMagicString(buf.to_vec()));
    }
    Ok(())
}

pub fn fetch_major_version(f: &mut std::fs::File) -> Result<u8, ReadHeaderError> {
    let mut buf = [0u8; SIZE_MAJOR_VERSION];
    read_exact(f, &mut buf)?;
    let major_version = buf[0];
    if !matches!(major_version, 1u8..=3u8) {
        return Err(ReadHeaderError::InvalidMajorVersion(major_version));
    }
    Ok(major_version)
}

pub fn check_minor_version(f: &mut std::fs::File) -> Result<(), ReadHeaderError> {
    let mut buf = [0u8; SIZE_MINOR_VERSION];
    read_exact(f, &mut buf)?;
    let minor_version = buf[0];
    if minor_version != 0u8 {
        return Err(ReadHeaderError::InvalidMinorVersion(minor_version));
    }
    Ok(())
}

pub fn fetch_header_len(
    f: &mut std::fs::File,
    major_version: u8,
) -> Result<usize, ReadHeaderError> {
    let header_len: usize = if major_version == 1u8 {
        let mut buf = [0u8; SIZE_HEADER_LEN[0]];
        read_exact(f, &mut buf)?;
        u16::from_le_bytes(buf) as usize
    } else {
        let mut buf = [0u8; SIZE_HEADER_LEN[1]];
        read_exact(f, &mut buf)?;
        u32::from_le_bytes(buf) as usize
    };
    let header_size: usize = match major_version {
        1u8 => {
            MAGIC_STRING.len()
                + SIZE_MAJOR_VERSION
                + SIZE_MINOR_VERSION
                + SIZE_HEADER_LEN[0]
                + header_len
        }
        2u8 | 3u8 => {
            MAGIC_STRING.len()
                + SIZE_MAJOR_VERSION
                + SIZE_MINOR_VERSION
                + SIZE_HEADER_LEN[1]
                + header_len
        }
        _ => return Err(ReadHeaderError::InvalidMajorVersion(major_version)),
    };
    if header_size % HEADER_BLOCK_SIZE != 0usize {
        return Err(ReadHeaderError::InvalidHeaderSize(header_size));
    }
    Ok(header_len)
}

pub fn fetch_header(
    f: &mut std::fs::File,
    header_len: usize,
) -> Result<crate::Header, ReadHeaderError> {
    let header: crate::Header = {
        let mut buf = vec![0u8; header_len];
        read_exact(f, &mut buf)?;
        dict::parse(&buf)?
    };
    Ok(header)
}
