mod consts;
pub mod error;
mod reader;
mod writer;

pub struct Header {
    pub descr: String,
    pub fortran_order: bool,
    pub shape: Vec<usize>,
}

pub fn read_header(f: &mut std::fs::File) -> Result<Header, error::ReadHeaderError> {
    reader::check_magic_string(f)?;
    let major_version: u8 = reader::fetch_major_version(f)?;
    reader::check_minor_version(f)?;
    let header_len: usize = reader::fetch_header_len(f, major_version)?;
    let header: Header = reader::fetch_header(f, header_len)?;
    Ok(header)
}

pub fn write_header(f: &mut std::fs::File, header: &Header) -> Result<(), error::WriteHeaderError> {
    let dict: Vec<u8> = writer::prepare_dictionary(header)?;
    let buffer_info: writer::BufferInfo = writer::prepare_buffer_info(dict.len());
    let padding: Vec<u8> = writer::prepare_padding(&buffer_info)?;
    writer::write_all(f, &buffer_info, &dict, &padding)?;
    Ok(())
}
