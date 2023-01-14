use varint::Varint;

pub struct CID {
    version: u64,
    codec: u64,
    hash: Vec<u8>,
}

impl CID {
    pub fn new(version: &str, codec: &str, hash: Vec<u8>) -> Self {
        let version = multicodec::to_code(version);
        let codec = multicodec::to_code(codec);

        Self {
            version,
            codec,
            hash,
        }
    }

    pub fn encode<T: multibase::Encoding>(&self) -> String {
        let version_len = varint::required_encode_space(self.version);
        let codec_len = varint::required_encode_space(self.codec);
        let hash_len = self.hash.len();

        let mut cnt = Vec::with_capacity(version_len + codec_len + hash_len);
        unsafe { cnt.set_len(version_len + codec_len + hash_len) };

        self.version.encode_varint(cnt.as_mut_slice());
        self.codec
            .encode_varint(&mut cnt.as_mut_slice()[version_len..]);
        cnt.as_mut_slice()[(version_len + codec_len)..].copy_from_slice(&self.hash);

        T::encode(&cnt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cidencode() {
        let hash = multihash::multihash("sha2-256", b"beep boop");
        let cid = CID::new("cidv1", "raw", hash);

        println!("{}", cid.encode::<multibase::Base32>())
    }
}
