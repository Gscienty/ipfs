mod bigint;
mod block_encode;
mod encoding;
mod encoding_declare;

mod prouints;

use block_encode::*;
use encoding_declare::*;

pub use encoding::*;
pub use prouints::Prouints;

pub trait Encoder {
    fn encode(input: &[u8]) -> String;
}

pub trait Decoder {
    fn decode(input: &str) -> Vec<u8>;
}

macro_rules! match_encode {
    ($input: expr => $($prefix: expr => $base: ident);*;) => {
        match $input.chars().next() {
            $(
                Some($prefix) => $base::decode($input),
            )*
            _ => Vec::new(),
        }
    };
}

pub fn multibase_decode(input: &str) -> Vec<u8> {
    match_encode!(input =>
        '0' => Base2;
        '7' => Base8;

        '9' => Base10;

        'f' => Base16;
        'F' => Base16Upper;

        'v' => Base32Hex;
        'V' => Base32HexUpper;
        't' => Base32HexPad;
        'T' => Base32HexPadUpper;

        'b' => Base32;
        'B' => Base32Upper;
        'c' => Base32Pad;
        'C' => Base32PadUpper;

        'h' => Base32Z;

        'k' => Base36;
        'K' => Base36Upper;

        'z' => Base58Bitcoin;
        'Z' => Base58Flickr;

        'm' => Base64;
        'M' => Base64Pad;
        'u' => Base64URL;
        'U' => Base64URLPad;

        '🚀' => Base256Emoji;
    )
}
