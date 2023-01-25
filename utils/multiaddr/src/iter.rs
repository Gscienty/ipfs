use crate::Protocol;

pub struct ProtocolIter<'s>(pub(crate) &'s [u8]);

impl<'s> Iterator for ProtocolIter<'s> {
    type Item = Protocol<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        let (protocol, len) = Protocol::from_bytes(self.0).expect("invalid multiaddr");
        self.0 = &self.0[len..];

        Some(protocol)
    }
}
