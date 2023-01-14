use std::slice::{from_raw_parts, from_raw_parts_mut};

use crate::Encoding;

pub(crate) struct ProuintsBlockEncoding {
    mse: bool,
}

impl ProuintsBlockEncoding {
    pub(crate) fn new() -> Self {
        Self { mse: true }
    }

    #[inline]
    fn inorder(&self, i: usize) -> usize {
        if self.mse {
            1 - i
        } else {
            i
        }
    }

    #[inline]
    fn outorder(&self, i: usize) -> usize {
        if self.mse {
            4 - i
        } else {
            i
        }
    }

    #[inline]
    pub(crate) fn encode_len(input_bytes: usize) -> usize {
        (input_bytes + 1) / 2 * 5
    }

    fn consonant(v: u16) -> char {
        vec![
            'b', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'r', 's', 't', 'v', 'z',
        ][v as usize]
    }

    fn vowel(v: u16) -> char {
        vec!['a', 'i', 'o', 'u'][v as usize]
    }

    pub(crate) fn encode(&self, input: &[u8], output: &mut [char]) {
        debug_assert!(input.len() <= 2);
        debug_assert_eq!(output.len(), 5);
        let mut x = 0u16;
        for (index, input) in input.iter().enumerate() {
            x |= u16::from(*input) << (8 * self.inorder(index));
        }

        output[self.outorder(0)] = Self::consonant(x & 0x000f);
        output[self.outorder(1)] = Self::vowel((x >> 4) & 0x0003);
        output[self.outorder(2)] = Self::consonant((x >> 6) & 0x000f);
        output[self.outorder(3)] = Self::vowel((x >> 10) & 0x0003);
        output[self.outorder(4)] = Self::consonant((x >> 12) & 0x000f);
    }
}

pub struct Prouints;

fn inner_encode(input: &[u8], output: &mut [char]) {
    let blk = ProuintsBlockEncoding::new();
    let nblk = input.len() / 2;

    for grp in 0..nblk {
        let input = unsafe { from_raw_parts(input.as_ptr().add(grp * 2), 2) };
        let output = unsafe { from_raw_parts_mut(output.as_mut_ptr().add(grp * 6), 6) };

        blk.encode(input, &mut output[..5]);
        output[5] = '-';
    }

    if input.len() % 2 != 0 {
        let input = vec![*input.last().unwrap(), 0u8];

        blk.encode(&input, &mut output[nblk * 6..(nblk + 1) * 6 - 1]);
    }
}

impl Encoding for Prouints {
    fn encode(input: &[u8]) -> String {
        let len = ProuintsBlockEncoding::encode_len(input.len());

        let mut output = Vec::with_capacity(len + len / 5 + 1);
        unsafe { output.set_len(len + len / 5 + 1) }

        output[0] = 'p';
        inner_encode(input, &mut output.as_mut_slice()[1..]);

        unsafe { output.set_len(len + len / 5) };
        output.iter().collect()
    }
}
