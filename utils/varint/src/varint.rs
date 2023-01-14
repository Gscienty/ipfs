pub trait Varint: Copy + Sized {
    fn encode_varint(self, dst: &mut [u8]) -> usize;
    fn decode_varint(src: &[u8]) -> Option<(Self, usize)>;
}

impl Varint for u64 {
    fn encode_varint(self, dst: &mut [u8]) -> usize {
        let mut n = self;
        let mut i = 0;

        while n >= 0x80 {
            dst[i] = 0b1000_0000 | (n as u8);
            i += 1;
            n >>= 7;
        }

        dst[i] = n as u8;
        i + 1
    }

    fn decode_varint(src: &[u8]) -> Option<(Self, usize)> {
        let mut result = 0u64;
        let mut shift = 0;

        let mut success = false;
        for b in src.iter() {
            result |= ((b & 0b0111_1111) as u64) << shift;
            shift += 7;

            if (b & 0b1000_0000) == 0 || shift > (9 * 7) {
                success = (b & 0b1000_0000) == 0;
                break;
            }
        }

        if success {
            Some((result, shift / 7 as usize))
        } else {
            None
        }
    }
}
