mod dict;
pub mod error;

use crate::consts::{
    HEADER_BLOCK_SIZE, MAGIC_STRING, MAX_HEADER_SIZE_V1, MINOR_VERSION, SIZE_HEADER_LEN,
    SIZE_MAJOR_VERSION, SIZE_MINOR_VERSION,
};
use crate::error::WriteHeaderError;
use std::io::Write;

pub struct BufferInfo {
    major_version: u8,
    header_len: Vec<u8>,
    padding_size: usize,
}

pub fn prepare_dictionary(header: &crate::Header) -> Result<Vec<u8>, WriteHeaderError> {
    let descr: String = dict::prepare_descr(&header.descr)?;
    let fortran_order: String = dict::prepare_fortran_order(header.fortran_order)?;
    let shape: String = dict::prepare_shape(&header.shape)?;
    let dict: String = format!(
        r#"{{'descr':{},'fortran_order':{},'shape':{}}}"#,
        descr, fortran_order, shape
    );
    Ok(dict.into_bytes())
}

pub fn prepare_buffer_info(dict_len: usize) -> BufferInfo {
    let mut header_size = HEADER_BLOCK_SIZE;
    while header_size <= dict_len {
        header_size += HEADER_BLOCK_SIZE;
    }
    let major_version = if header_size <= MAX_HEADER_SIZE_V1 {
        1u8
    } else {
        2u8
    };
    let header_len: usize = header_size
        - MAGIC_STRING.len()
        - SIZE_MAJOR_VERSION
        - SIZE_MINOR_VERSION
        - SIZE_HEADER_LEN[if major_version == 1u8 { 0usize } else { 1usize }];
    let padding_size: usize = header_len - dict_len;
    let header_len_bytes: Vec<u8> = if major_version == 1u8 {
        (header_len as u16).to_le_bytes().to_vec()
    } else {
        (header_len as u32).to_le_bytes().to_vec()
    };
    BufferInfo {
        major_version,
        header_len: header_len_bytes,
        padding_size,
    }
}

pub fn prepare_padding(buffer_info: &BufferInfo) -> Result<Vec<u8>, WriteHeaderError> {
    let padding_size: usize = buffer_info.padding_size;
    if padding_size == 0usize {
        return Err(WriteHeaderError::ZeroPaddingSize);
    }
    let mut buf = vec![0x20u8; padding_size.saturating_sub(1)];
    buf.push(0x0au8);
    Ok(buf)
}

pub fn write_all(
    f: &mut std::fs::File,
    buffer_info: &BufferInfo,
    dict: &[u8],
    padding: &[u8],
) -> Result<(), WriteHeaderError> {
    let mut buf = Vec::<u8>::new();
    buf.extend_from_slice(MAGIC_STRING);
    buf.extend_from_slice(&[buffer_info.major_version]);
    buf.extend_from_slice(&[MINOR_VERSION]);
    buf.extend_from_slice(&buffer_info.header_len);
    buf.extend_from_slice(dict);
    buf.extend_from_slice(padding);
    f.write_all(&buf)?;
    Ok(())
}
