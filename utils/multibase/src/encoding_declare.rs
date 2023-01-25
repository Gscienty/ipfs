use crate::BlockEncoding;
use std::slice::{from_raw_parts, from_raw_parts_mut};

pub(crate) fn inner_encode(blk: &BlockEncoding<char>, input: &[u8], output: &mut [char]) {
    let nblk = input.len() / blk.enc_len;
    for grp in 0..nblk {
        let input = unsafe { from_raw_parts(input.as_ptr().add(grp * blk.enc_len), blk.enc_len) };
        let output =
            unsafe { from_raw_parts_mut(output.as_mut_ptr().add(grp * blk.dec_len), blk.dec_len) };

        blk.encode(input, output);
    }

    blk.encode(
        &input[nblk * blk.enc_len..],
        &mut output[nblk * blk.dec_len..],
    );
}

pub(crate) fn inner_decode(blk: &BlockEncoding<char>, input: &[char], output: &mut [u8]) {
    let nblk = input.len() / blk.dec_len;
    for grp in 0..nblk {
        let input = unsafe { from_raw_parts(input.as_ptr().add(grp * blk.dec_len), blk.dec_len) };
        let output =
            unsafe { from_raw_parts_mut(output.as_mut_ptr().add(grp * blk.enc_len), blk.enc_len) };

        blk.decode(input, output);
    }

    blk.decode(
        &input[nblk * blk.dec_len..],
        &mut output[nblk * blk.enc_len..],
    )
}

#[macro_export]
macro_rules! encoding_declare {
    ($encoding: ident $(, prefix: $prefix: expr)? => $symbols: expr $(,$($opt: expr),+)?) => {
        pub struct $encoding;

        impl $crate::Encoder for $encoding {
            fn encode(input: &[u8]) -> String {
                let symbols: Vec<char> = $symbols.chars().collect();

                #[allow(unused_mut)]
                let mut blk = BlockEncoding::new(&symbols);
                $($(
                        $opt(&mut blk);
                )+)?

                let len = blk.encode_len(input.len()) $(+ {$prefix; 1})?;
                let padding_len = blk.dec_len - len % blk.dec_len;

                let mut output = Vec::with_capacity(len + padding_len);
                unsafe { output.set_len(len) };

                #[allow(unused_mut)]
                let mut output_slice = output.as_mut_slice();
                $(output_slice = {output_slice[0] = $prefix; &mut output_slice[1..]};)?

                $crate::inner_encode(&blk, input, output_slice);

                if let Some(padding) = blk.padding {
                    output.resize(output.len() + padding_len, padding);
                }

                output.iter().collect()
            }
        }

        impl $crate::Decoder for $encoding {
            fn decode(input: &str) -> Vec<u8> {
                let symbols: Vec<char> = $symbols.chars().collect();

                #[allow(unused_mut)]
                let mut blk = BlockEncoding::new(&symbols);
                $($(
                    $opt(&mut blk);
                )+)?

                let (_, len) = blk.decode_len(input.len() - 1);
                let mut output = Vec::with_capacity(len);
                unsafe { output.set_len(len) };

                let input: Vec<char> = input.chars().skip(1).collect();
                $crate::inner_decode(&blk, &input, &mut output);

                output
            }
        }
    };
}

#[macro_export]
macro_rules! encoding_x_declare {
    ($encoding: ident $(, prefix: $prefix: expr)? => $symbols: expr) => {
        pub struct $encoding;

        impl $crate::Encoder for $encoding {
            fn encode(input: &[u8]) -> String {
                let symbols: Vec<char> = $symbols.chars().collect();
                let base = symbols.len() as u32;
                let big_pow = 32 / (32 - base.leading_zeros());
                let big_base = base.pow(big_pow);

                let mut output = Vec::with_capacity(input.len()$(+{$prefix; 1})?);
                $(output.push($prefix);)?

                let mut big = Bigint::from_bytes(input);
                'fast: loop {
                    let mut big_remain = big.modulo(big_base);

                    if big.is_zero() {
                        loop {
                            let (result, remain) = (big_remain / base, big_remain % base);
                            output.push(symbols[remain as usize]);
                            big_remain = result;

                            if big_remain == 0 {
                                break 'fast;
                            }
                        }
                    } else {
                        for _ in 0..big_pow {
                            let (result, remain) = (big_remain / base, big_remain % base);
                            output.push(symbols[remain as usize]);
                            big_remain = result;
                        }
                    }
                }

                let leaders = input
                    .iter()
                    .take(input.len() - 1)
                    .take_while(|i| **i == 0)
                    .map(|_| symbols[0]);
                output.extend(leaders);

                output.iter().collect()
            }
        }

        impl $crate::Decoder for $encoding {
            fn decode(input: &str) -> Vec<u8> {
                let mut rsymbols = HashMap::new();
                for (index, symbol) in $symbols.chars().enumerate() {
                    rsymbols.insert(symbol, index as u32);
                }

                let base = $symbols.len() as u32;
                let mut big = Bigint::from_bytes(&vec![0, 0, 0, 0]);
                for c in input.chars().skip(1) {
                    if let Some(carry) = rsymbols.get(&c) {
                        big.mul_add(base, *carry);
                    } else {
                        return Vec::new();
                    }
                }

                let mut bytes = big.into_bytes();
                let leader = match $symbols.chars().next() {
                    Some(leader) => leader,
                    _ => return Vec::new(),
                };
                let leaders = input.chars().take_while(|byte| *byte == leader).count();

                for _ in 0..leaders {
                    bytes.insert(0, 0);
                }

                bytes
            }
        }
    };
}
