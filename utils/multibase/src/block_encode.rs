pub(crate) struct BlockEncoding<'s, T: Copy> {
    symbols: &'s [T],
    bit: usize,
    pub(crate) inblk_len: usize,
    pub(crate) outblk_len: usize,
    pub(crate) padding: Option<T>,
    mse: bool,
}

impl<'s, T: Copy> BlockEncoding<'s, T> {
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

        BlockEncoding {
            symbols,
            bit,
            inblk_len,
            outblk_len: inblk_len * 8 / bit,
            padding: None,
            mse: false,
        }
    }

    #[inline]
    pub(crate) fn encode_len(&self, input_bytes: usize) -> usize {
        (input_bytes * 8 + self.bit - 1) / self.bit
    }

    #[inline]
    fn inorder(&self, i: usize) -> usize {
        if self.mse {
            self.inblk_len - 1 - i
        } else {
            i
        }
    }

    #[inline]
    fn outorder(&self, i: usize) -> usize {
        if self.mse {
            self.outblk_len - 1 - i
        } else {
            i
        }
    }

    pub(crate) fn encode(&self, input: &[u8], output: &mut [T]) {
        debug_assert!(input.len() <= self.inblk_len);
        debug_assert_eq!(output.len(), self.encode_len(input.len()));

        let mut x = 0u64;
        for (index, input) in input.iter().enumerate() {
            x |= u64::from(*input) << (8 * self.inorder(index));
        }

        for (index, output) in output.iter_mut().enumerate() {
            let y = x >> (self.bit * self.outorder(index));
            *output = self.symbols[y as usize % (1 << self.bit)];
        }
    }
}

pub(crate) fn use_big_endian<T: Copy>() -> fn(&mut BlockEncoding<T>) {
    |blk| blk.mse = true
}

pub(crate) fn use_padding<T: Copy>(chr: T) -> impl Fn(&mut BlockEncoding<T>) {
    move |blk: &mut BlockEncoding<T>| blk.padding = Some(chr)
}
