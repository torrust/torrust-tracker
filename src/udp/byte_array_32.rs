use std::ops::BitOr;
use arraytools::ArrayTools;
use std::convert::From;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ByteArray32([u8; 32]);

impl ByteArray32 {

    pub fn new(bytes: [u8; 32]) -> Self {
        ByteArray32(bytes)
    }

    pub fn as_generic_byte_array(self) -> [u8; 32] {
        self.0
    }
}

impl BitOr for ByteArray32 {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a | b`
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0.zip_with(rhs.0, BitOr::bitor))
    }
}

impl From<u64> for ByteArray32 {
    fn from(item: u64) -> Self {
        let vec: Vec<u8> = [
            [0u8; 24].as_slice(),
            item.to_be_bytes().as_slice(), // 8 bytes
        ].concat();

        let bytes: [u8; 32] = match vec.try_into() {
            Ok(bytes) => bytes,
            Err(_) => panic!("Expected a Vec of length 32"),
        };

        Self(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_instantiated_from_an_u64() {

        // Pad numbers with zeros on the left

        assert_eq!(ByteArray32::from(0x00_00_00_00_00_00_00_00_u64), ByteArray32::new([
            //  0    1    2    3    4    5    6    7    8    9   10   11   12   13   14   15   16   17   18   19   20   21   22   23
            [ 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0].as_slice(), //   24 bytes
            [ 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0].as_slice(),                                                                                 // +  8 bytes (64 bits, u64)
            ].concat().try_into().unwrap()));                                                                                                     //   32 bytes

        assert_eq!(ByteArray32::from(0xFF_FF_FF_FF_FF_FF_FF_00_u64), ByteArray32::new([
            //  0      1     2     3     4     5     6     7    8    9   10   11   12   13   14   15   16   17   18   19   20   21   22   23
            [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0].as_slice(), //   24 bytes
            [ 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00].as_slice(),                                                                                 // +  8 bytes (64 bits, u64)
            ].concat().try_into().unwrap()));                                                                                                             //   32 bytes                                                           //   32 bytes
    }

    #[test]
    fn it_should_be_converted_into_a_generic_byte_array() {

        let byte_array_32 = ByteArray32::new([0; 32]);

        assert_eq!(byte_array_32.as_generic_byte_array(), [0u8; 32]);
    }    

    #[test]
    fn it_should_support_bitwise_or_operator() {
        assert_eq!(ByteArray32::new([   0; 32]) | ByteArray32::new([   0; 32]), ByteArray32::new([   0; 32])); // 0 | 0 = 0
        assert_eq!(ByteArray32::new([   0; 32]) | ByteArray32::new([0xFF; 32]), ByteArray32::new([0xFF; 32])); // 0 | 1 = 1
        assert_eq!(ByteArray32::new([0xFF; 32]) | ByteArray32::new([   0; 32]), ByteArray32::new([0xFF; 32])); // 1 | 0 = 1
        assert_eq!(ByteArray32::new([0xFF; 32]) | ByteArray32::new([0xFF; 32]), ByteArray32::new([0xFF; 32])); // 1 | 1 = 1
    }
}