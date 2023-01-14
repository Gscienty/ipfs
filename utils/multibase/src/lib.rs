mod bigint;
mod block_encode;
mod encoding;
mod encoding_declare;

mod prouints;

use block_encode::*;
use encoding_declare::*;

pub use encoding::*;
pub use prouints::Prouints;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_base4() {
        let output = Prouints::encode(vec![127u8, 0, 0, 1].as_slice());

        println!("{}, {}", output, output.as_bytes().len());
    }
}
