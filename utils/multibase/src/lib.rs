mod bigint;
mod block_encode;
mod encoding;
mod encoding_declare;

mod prouints;

use block_encode::*;
use encoding_declare::*;

pub use encoding::*;
pub use prouints::Prouints;

pub trait Encoding {
    fn encode(input: &[u8]) -> String;
}
