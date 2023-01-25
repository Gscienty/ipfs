use std::{
    collections::HashMap,
    slice::{from_raw_parts, from_raw_parts_mut},
};

use crate::{Decoder, Encoder};

pub(crate) struct ProuintsBlockEncoding {
    mse: bool,
}

impl ProuintsBlockEncoding {
    pub(crate) fn new() -> Self {
        Self { mse: true }
    }

    #[inline]
    fn dec_order(&self, i: usize) -> usize {
        if self.mse {
            1 - i
        } else {
            i
        }
    }

    #[inline]
    fn enc_order(&self, i: usize) -> usize {
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

    #[inline]
    pub(crate) fn decode_len(input_chars: usize) -> usize {
        (input_chars + 1) / 6 * 2
    }

    pub(crate) fn encode(&self, input: &[u8], output: &mut [char]) {
        debug_assert!(input.len() <= 2);
        debug_assert_eq!(output.len(), 5);
        let mut x = 0u16;
        for (index, input) in input.iter().enumerate() {
            x |= u16::from(*input) << (8 * self.dec_order(index));
        }

        let consonant = vec![
            'b', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'r', 's', 't', 'v', 'z',
        ];
        let vowel = vec!['a', 'i', 'o', 'u'];

        output[self.enc_order(0)] = consonant[(x & 0x000f) as usize];
        output[self.enc_order(1)] = vowel[((x >> 4) & 0x0003) as usize];
        output[self.enc_order(2)] = consonant[((x >> 6) & 0x000f) as usize];
        output[self.enc_order(3)] = vowel[((x >> 10) & 0x0003) as usize];
        output[self.enc_order(4)] = consonant[((x >> 12) & 0x000f) as usize];
    }

    pub(crate) fn decode(&self, input: &[char], output: &mut [u8]) {
        debug_assert!(output.len() <= 2);
        debug_assert_eq!(input.len(), 5);

        let mut consonant: HashMap<char, u8> = HashMap::new();
        consonant.insert('b', 0);
        consonant.insert('d', 1);
        consonant.insert('f', 2);
        consonant.insert('g', 3);
        consonant.insert('h', 4);
        consonant.insert('j', 5);
        consonant.insert('k', 6);
        consonant.insert('l', 7);
        consonant.insert('m', 8);
        consonant.insert('n', 9);
        consonant.insert('p', 10);
        consonant.insert('r', 11);
        consonant.insert('s', 12);
        consonant.insert('t', 13);
        consonant.insert('v', 14);
        consonant.insert('z', 15);

        let mut vowel: HashMap<char, u8> = HashMap::new();
        vowel.insert('a', 0);
        vowel.insert('i', 1);
        vowel.insert('o', 2);
        vowel.insert('u', 3);

        let mut x = 0u16;
        x |= match input.get(self.enc_order(0)).and_then(|x| consonant.get(x)) {
            Some(c) => *c as u16,
            _ => return,
        };
        x |= match input.get(self.enc_order(1)).and_then(|x| vowel.get(x)) {
            Some(c) => *c as u16,
            _ => return,
        } << 4;
        x |= match input.get(self.enc_order(2)).and_then(|x| consonant.get(x)) {
            Some(c) => *c as u16,
            _ => return,
        } << 6;
        x |= match input.get(self.enc_order(3)).and_then(|x| vowel.get(x)) {
            Some(c) => *c as u16,
            _ => return,
        } << 10;
        x |= match input.get(self.enc_order(4)).and_then(|x| consonant.get(x)) {
            Some(c) => *c as u16,
            _ => return,
        } << 12;

        output[self.dec_order(0)] = (x & 0xff) as u8;
        output[self.dec_order(1)] = ((x >> 8) & 0xff) as u8;
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

fn inner_decode(input: &[char], output: &mut [u8]) {
    let blk = ProuintsBlockEncoding::new();
    let nblk = (input.len() + 1) / 6;

    for grp in 0..nblk {
        let input = unsafe { from_raw_parts(input.as_ptr().add(grp * 6), 6) };
        let output = unsafe { from_raw_parts_mut(output.as_mut_ptr().add(grp * 2), 2) };

        blk.decode(&input[..5], output);
    }
}

impl Encoder for Prouints {
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

impl Decoder for Prouints {
    fn decode(input: &str) -> Vec<u8> {
        let len = ProuintsBlockEncoding::decode_len(input.len() - 1);

        let mut output = Vec::with_capacity(len);
        unsafe { output.set_len(len) };

        let chars: Vec<char> = input.chars().skip(1).map(|b| b).collect();
        inner_decode(&chars, &mut output);

        output
    }
}
