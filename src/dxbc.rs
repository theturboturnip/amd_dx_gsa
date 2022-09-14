use std::convert::TryInto;

///! Based on [https://github.com/baldurk/renderdoc/blob/4dc80c1793d2a068b6b7c4f76c8b084972d0e234/renderdoc/driver/ihv/amd/amd_isa_win32.cpp](https://github.com/baldurk/renderdoc/blob/4dc80c1793d2a068b6b7c4f76c8b084972d0e234/renderdoc/driver/ihv/amd/amd_isa_win32.cpp)
use nom::{
    bytes::complete::{tag, take},
    error::{ErrorKind, ParseError},
    multi::{count, length_data},
    number::complete::le_u32,
    sequence::tuple,
    Err::Error,
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum DXBCError<I> {
    NoUsableData { dxil: bool },
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for DXBCError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        DXBCError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

/// Parses the DXBC file and returns the contents of the first valid bytecode chunk (SHEX, SHDR).
/// If no such chunk is encountered, DXBCError::NoUsableData is returned.
/// DXIL-based chunks are not usable for our shader decompilation, so they are not returned when found.
///
/// Returns (full dxbc contents, first bytecode chunk contents)
pub fn get_shader_bytecode<'a>(
    dxbc_bytes: &'a [u8],
) -> IResult<&'a [u8], &'a [u8], DXBCError<&'a [u8]>> {
    let (input, (_magic, _hash, _unk, _length, num_chunks)) =
        tuple((tag(b"DXBC"), take(16usize), le_u32, le_u32, le_u32))(dxbc_bytes)?;
    let num_chunks: usize = num_chunks.try_into().unwrap();
    let (_, chunk_offsets) = count(le_u32, num_chunks)(input)?;

    let mut dxil = false;
    for chunk_offset in chunk_offsets {
        let chunk_offset: usize = chunk_offset.try_into().unwrap();
        let chunk = &dxbc_bytes[chunk_offset..];

        let (input, chunk_magic) = take(4usize)(chunk)?;

        match chunk_magic {
            b"SHEX" | b"SHDR" => {
                let (_, shader_bytecode) = length_data(le_u32)(input)?;
                return Ok((dxbc_bytes, shader_bytecode));
            }
            b"DXIL" | b"ILDB" => {
                dxil = true;
            }
            _ => {}
        }
    }

    // no usable shader data chunks
    return Err(Error(DXBCError::NoUsableData { dxil }));
}

#[cfg(test)]
mod test {
    use super::get_shader_bytecode;

    #[test]
    fn test_extract() {
        let full = include_bytes!("../assets/example.fxc");

        let expected_shaderbytes = &full[0xCC..0x104];

        let (_, actual_shaderbytes) = get_shader_bytecode(full).unwrap();

        assert_eq!(actual_shaderbytes, expected_shaderbytes)
    }
}
