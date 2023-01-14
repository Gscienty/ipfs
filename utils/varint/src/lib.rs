mod varint;

pub use varint::*;

pub fn required_encode_space(mut v: u64) -> usize {
    if v == 0 {
        return 1;
    }

    let mut bytes = 0;
    while v > 0 {
        bytes += 1;
        v >>= 7;
    }

    bytes
}
