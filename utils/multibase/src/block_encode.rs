use std::{collections::HashMap, hash::Hash};

pub(crate) struct BlockEncoding<'s, T: Copy + Hash + Eq> {
    symbols: &'s [T],
    rsymbols: HashMap<T, u64>,
    bit: usize,
    pub(crate) enc_len: usize,
    pub(crate) dec_len: usize,
    pub(crate) padding: Option<T>,
    mse: bool,
}

impl<'s, T: Copy + Hash + Eq> BlockEncoding<'s, T> {
    pub(crate) fn new(symbols: &'s [T]) -> Self {
        let (bit, inblk_len) = match symbols.len() {
            2 => (1, 1),
            4 => (2, 1),
            8 => (3, 3),
            16 => (4, 1),
            32 => (5, 5),
            64 => (6, 3),
            128 => (7, 7),
            256 => (8, 1),
            _ => unreachable!(),
        };

        let mut rsymbols = HashMap::new();
        for (index, symbol) in symbols.iter().enumerate() {
            rsymbols.insert(*symbol, index as u64);
        }

        BlockEncoding {
            symbols,
            rsymbols,
            bit,
            enc_len: inblk_len,
            dec_len: inblk_len * 8 / bit,
            padding: None,
            mse: false,
        }
    }

    #[inline]
    pub(crate) fn encode_len(&self, input_bytes: usize) -> usize {
        (input_bytes * 8 + self.bit - 1) / self.bit
    }

    // return: input len, output len
    #[inline]
    pub(crate) fn decode_len(&self, input_chars: usize) -> (usize, usize) {
        if self.padding.is_some() {
            (
                input_chars / self.dec_len * self.dec_len,
                input_chars / self.dec_len * self.enc_len,
            )
        } else {
            (
                input_chars - (self.bit * input_chars % 8) / self.bit,
                self.bit * input_chars / 8,
            )
        }
    }

    #[inline]
    fn enc_order(&self, i: usize) -> usize {
        if self.mse {
            self.enc_len - 1 - i
        } else {
            i
        }
    }

    #[inline]
    fn dec_order(&self, i: usize) -> usize {
        if self.mse {
            self.dec_len - 1 - i
        } else {
            i
        }
    }

    pub(crate) fn encode(&self, input: &[u8], output: &mut [T]) {
        debug_assert!(input.len() <= self.enc_len);
        debug_assert_eq!(output.len(), self.encode_len(input.len()));

        let mut x = 0u64;
        for (index, input) in input.iter().enumerate() {
            x |= u64::from(*input) << (8 * self.enc_order(index));
        }

        for (index, output) in output.iter_mut().enumerate() {
            let y = x >> (self.bit * self.dec_order(index));
            *output = self.symbols[y as usize % (1 << self.bit)];
        }
    }

    pub(crate) fn decode(&self, input: &[T], output: &mut [u8]) {
        debug_assert!(output.len() <= self.enc_len);
        debug_assert_eq!(input.len(), self.encode_len(output.len()));

        let mut x = 0u64;
        for i in 0..input.len() {
            let y = self.rsymbols[&input[i]];
            x |= y << (self.bit * self.dec_order(i));
        }
        for (i, output) in output.iter_mut().enumerate() {
            *output = (x >> (8 * self.enc_order(i))) as u8;
        }
    }
}

pub(crate) fn use_big_endian<T: Copy + Hash + Eq>() -> fn(&mut BlockEncoding<T>) {
    |blk| blk.mse = true
}

pub(crate) fn use_padding<T: Copy + Hash + Eq>(chr: T) -> impl Fn(&mut BlockEncoding<T>) {
    move |blk: &mut BlockEncoding<T>| blk.padding = Some(chr)
}
