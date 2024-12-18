pub const MAGIC_STRING: &[u8] = &[0x93, b'N', b'U', b'M', b'P', b'Y'];
pub const SIZE_MAJOR_VERSION: usize = 1usize;
pub const SIZE_MINOR_VERSION: usize = 1usize;
// for now minor version is fixed to 0
pub const MINOR_VERSION: u8 = 0u8;
// differ depending on the major version
pub const SIZE_HEADER_LEN: [usize; 2] = [2usize, 4usize];
pub const HEADER_BLOCK_SIZE: usize = 64usize;
#[cfg(feature = "writer")]
pub const MAX_HEADER_SIZE_V1: usize = 65535usize;
#[cfg(feature = "writer")]
pub const ENDIAN_SPECIFIERS: &[char] = &['<', '>', '=', '|'];
