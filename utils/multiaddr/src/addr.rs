use std::io;

use crate::{Protocol, ProtocolIter};

pub struct MultiAddr {
    bytes: Vec<u8>,
}

impl MultiAddr {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        Vec::from(&self.bytes[..])
    }

    pub fn push(&mut self, protocol: Protocol<'_>) {
        let mut w = io::Cursor::<&mut Vec<u8>>::new(&mut self.bytes);
        w.set_position(w.get_ref().len() as u64);

        protocol.write_bytes(&mut w).expect("writing failed");
    }

    pub fn pop<'s>(&mut self) -> Option<Protocol<'s>> {
        let mut slice = &self.bytes[..];
        if slice.is_empty() {
            return None;
        }

        let mut remain = 0;
        let protocol = loop {
            let (protocol, len) = Protocol::from_bytes(&slice).expect("from bytes failed");
            slice = &slice[len..];
            if slice.is_empty() {
                break protocol.acquire();
            }

            remain += len;
        };

        self.bytes.truncate(remain);
        Some(protocol)
    }

    pub fn iter(&self) -> ProtocolIter<'_> {
        ProtocolIter(&self.bytes)
    }
}
