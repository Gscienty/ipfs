extern crate openssl;

use digest::{DynDigest, OutputSizeUser};
use openssl::hash::{Hasher, MessageDigest};

macro_rules! hash {
    ($hashtype: expr, $input: expr, $output: expr) => {{
        let mut md = $hashtype;
        md.update($input);
        md.finalize_into($output).unwrap();
    }};
}

macro_rules! blake_hash {
    ($hash: expr, $hashtype: expr, $input: expr, $output: expr) => {{
        let mut params = $hash;
        params.hash_length(($hashtype & 0x003f) as usize);

        let mut state = params.to_state();
        state.update($input);
        let hash = state.finalize();

        $output.copy_from_slice(hash.as_bytes());
    }};
}

pub fn hash(hashtype: u64, input: &[u8], output: &mut [u8]) {
    match hashtype {
        0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x20 => {
            // use openssl

            let md = match hashtype {
                0x11 => MessageDigest::sha1(),
                0x12 => MessageDigest::sha256(),
                0x13 => MessageDigest::sha256(),
                0x14 => MessageDigest::sha3_512(),
                0x15 => MessageDigest::sha3_384(),
                0x16 => MessageDigest::sha3_256(),
                0x17 => MessageDigest::sha3_224(),
                0x18 => MessageDigest::shake_128(),
                0x19 => MessageDigest::shake_256(),
                0x20 => MessageDigest::sha3_384(),
                _ => unreachable!(),
            };

            debug_assert_eq!(output.len(), md.size());

            let mut hasher = Hasher::new(md).unwrap();
            hasher.update(input).unwrap();
            let finished = hasher.finish().unwrap();

            output.copy_from_slice(&finished);
        }

        0x1052 => hash!(ripemd::Ripemd128::default(), input, output),
        0x1053 => hash!(ripemd::Ripemd160::default(), input, output),
        0x1054 => hash!(ripemd::Ripemd256::default(), input, output),
        0x1055 => hash!(ripemd::Ripemd320::default(), input, output),

        0x1a => hash!(sha3::Keccak224::default(), input, output),
        0x1b => hash!(sha3::Keccak256::default(), input, output),
        0x1c => hash!(sha3::Keccak384::default(), input, output),
        0x1d => hash!(sha3::Keccak512::default(), input, output),

        0x1e => {
            let hash = blake3::hash(input);
            output.copy_from_slice(hash.as_bytes());
        }
        0xb201..=0xb240 => blake_hash!(blake2b_simd::Params::new(), hashtype, input, output),

        0xb241..=0xb260 => blake_hash!(blake2s_simd::Params::new(), hashtype, input, output),

        _ => unreachable!(),
    }
}

pub fn hash_len(hashtype: u64) -> usize {
    match hashtype {
        0x11 => MessageDigest::sha1().size(),
        0x12 => MessageDigest::sha256().size(),
        0x13 => MessageDigest::sha256().size(),
        0x14 => MessageDigest::sha3_512().size(),
        0x15 => MessageDigest::sha3_384().size(),
        0x16 => MessageDigest::sha3_256().size(),
        0x17 => MessageDigest::sha3_224().size(),
        0x18 => MessageDigest::shake_128().size(),
        0x19 => MessageDigest::shake_256().size(),
        0x20 => MessageDigest::sha3_384().size(),

        0x1052 => ripemd::Ripemd128Core::output_size(),
        0x1053 => ripemd::Ripemd160Core::output_size(),
        0x1054 => ripemd::Ripemd256Core::output_size(),
        0x1055 => ripemd::Ripemd320Core::output_size(),

        0x1a => sha3::Keccak224Core::output_size(),
        0x1b => sha3::Keccak256Core::output_size(),
        0x1c => sha3::Keccak384Core::output_size(),
        0x1d => sha3::Keccak512Core::output_size(),

        0x1e => blake3::OUT_LEN,

        0xb201..=0xb260 => (hashtype & 0x003f) as usize,

        _ => unreachable!(),
    }
}

pub fn digest(hashtype: u64, input: &[u8]) -> Vec<u8> {
    let len = hash_len(hashtype);
    let mut output = Vec::with_capacity(len);
    unsafe { output.set_len(len) };

    hash(hashtype, input, output.as_mut_slice());

    output
}

pub fn multihash(hashtype: &str, input: &[u8]) -> Vec<u8> {
    use varint::Varint;

    let hashtype = multicodec::to_code(hashtype);
    let len = hash_len(hashtype);

    let ht_len = varint::required_encode_space(hashtype);
    let dig_len = varint::required_encode_space(len as u64);

    let mut output = Vec::with_capacity(ht_len + dig_len + len);
    unsafe { output.set_len(ht_len + dig_len + len) };

    hashtype.encode_varint(&mut output.as_mut_slice()[..ht_len]);
    (len as u64).encode_varint(&mut output.as_mut_slice()[ht_len..(ht_len + dig_len)]);

    hash(
        hashtype,
        input,
        &mut output.as_mut_slice()[(ht_len + dig_len)..],
    );

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuck() {
        let output = multihash("sha2-512", "hello world".as_bytes());

        println!("{:?}", output);
    }
}
