use std::ptr::copy_nonoverlapping;

pub(crate) struct Bigint {
    chunks: Vec<u32>,
}

impl Bigint {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        let remain = bytes.len() % 4;
        let len = bytes.len() / 4 + (remain > 0) as usize;

        let mut chunks = Vec::with_capacity(len);

        unsafe {
            chunks.set_len(len);

            let mut chunks_ptr = chunks.as_mut_ptr() as *mut u8;

            if remain > 0 {
                *chunks.get_unchecked_mut(0) = 0u32;
                chunks_ptr = chunks_ptr.offset(4 - remain as isize);
            }

            copy_nonoverlapping(bytes.as_ptr(), chunks_ptr, bytes.len());
        }

        for chunk in chunks.iter_mut() {
            *chunk = u32::from_be(*chunk);
        }

        Self { chunks }
    }

    pub(crate) fn into_bytes(mut self) -> Vec<u8> {
        let mut skip = 0;

        for chunk in self.chunks.iter() {
            if *chunk != 0 {
                skip += chunk.leading_zeros() / 8;
                break;
            }
            skip += 4;
        }

        let len = self.chunks.len() * 4 - skip as usize;
        if len == 0 {
            return Vec::new();
        }

        for chunk in self.chunks.iter_mut() {
            *chunk = u32::to_be(*chunk);
        }

        let mut bytes = Vec::with_capacity(len);
        unsafe {
            bytes.set_len(len);
            let chunks_ptr = (self.chunks.as_ptr() as *const u8).offset(skip as isize);
            copy_nonoverlapping(chunks_ptr, bytes.as_mut_ptr(), len);
        }

        bytes
    }

    pub(crate) fn modulo(&mut self, divider: u32) -> u32 {
        let mut carry = 0u64;

        for chunk in self.chunks.iter_mut() {
            carry = (carry << 32) | u64::from(*chunk);
            *chunk = (carry / u64::from(divider)) as u32;
            carry %= u64::from(divider);
        }

        if let Some(0) = self.chunks.get(0) {
            self.chunks.remove(0);
        }

        carry as u32
    }

    pub(crate) fn mul_add(&mut self, mul: u32, add: u32) {
        let mut carry = 0u64;

        let mut iter = self.chunks.iter_mut().rev();
        if let Some(chunk) = iter.next() {
            carry = u64::from(*chunk) * u64::from(mul) + u64::from(add);
            *chunk = carry as u32;
            carry >>= 32;
        }
        for chunk in iter {
            carry += u64::from(*chunk) * u64::from(mul);
            *chunk = carry as u32;
            carry >>= 32;
        }

        if carry > 0 {
            self.chunks.insert(0, carry as u32);
        }
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.chunks.iter().all(|chunk| *chunk == 0)
    }
}
