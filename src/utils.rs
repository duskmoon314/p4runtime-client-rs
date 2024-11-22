//! utility types and functions

pub mod de;

/// Canonicalize a bitstring
///
/// P4Runtime uses a canonical form for bitstrings, i.e., the representation
/// should be the shortest possible.
///
/// This function takes a bitstring and returns the canonical form of it by
/// removing leading 0x00 or 0xFF bytes.
pub fn canonicalize_bitstring(bytes: Vec<u8>) -> Vec<u8> {
    if bytes.is_empty() {
        return bytes;
    }

    // Find the start of the first non 0x00/0xFF byte
    let start = bytes.iter().position(|&x| x != 0x00 && x != 0xFF);

    // If there is no such byte, return the last byte
    match start {
        Some(start) => bytes[start..].to_vec(),
        None => bytes.last().cloned().into_iter().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonicalize_bitstring() {
        assert_eq!(
            canonicalize_bitstring(99u8.to_be_bytes().to_vec()),
            vec![0x63]
        );
        assert_eq!(
            canonicalize_bitstring(99u16.to_be_bytes().to_vec()),
            vec![0x63]
        );
        assert_eq!(
            canonicalize_bitstring(12388u16.to_be_bytes().to_vec()),
            vec![0x30, 0x64]
        );
        assert_eq!(
            canonicalize_bitstring((-99i8).to_be_bytes().to_vec()),
            vec![0x9d]
        );
        assert_eq!(
            canonicalize_bitstring((-739i16).to_be_bytes().to_vec()),
            vec![0xfd, 0x1d]
        );
        assert_eq!(
            canonicalize_bitstring((0u16).to_be_bytes().to_vec()),
            vec![0x00]
        )
    }
}
