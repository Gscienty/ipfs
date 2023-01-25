use openssl::{error::ErrorStack, hash::MessageDigest, pkey, rsa, sign};

extern crate openssl;

pub struct RSAPriKey(pkey::PKey<pkey::Private>);

impl RSAPriKey {
    pub(crate) fn generate(bits: u32) -> Result<Self, ErrorStack> {
        let private_key = rsa::Rsa::generate(bits)?;

        Ok(Self(pkey::PKey::from_rsa(private_key)?))
    }

    pub(crate) fn public_key(&self) -> Result<RSAPubKey, ErrorStack> {
        let raw = self.0.raw_public_key()?;

        RSAPubKey::from_raw(&raw)
    }

    pub(crate) fn raw_key(&self) -> Vec<u8> {
        self.0.raw_private_key().unwrap()
    }

    pub(crate) fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, ErrorStack> {
        let mut signer = sign::Signer::new(MessageDigest::sha256(), self.0.as_ref())?;
        signer.update(msg)?;

        signer.sign_to_vec()
    }
}

pub struct RSAPubKey(pkey::PKey<pkey::Public>);

impl RSAPubKey {
    pub(crate) fn from_raw(raw: &[u8]) -> Result<Self, ErrorStack> {
        Ok(Self(pkey::PKey::public_key_from_raw_bytes(
            raw,
            pkey::Id::RSA,
        )?))
    }

    pub(crate) fn raw_key(&self) -> Vec<u8> {
        self.0.raw_public_key().unwrap()
    }

    pub(crate) fn verify(&self, msg: &[u8], sig: &[u8]) -> Result<bool, ErrorStack> {
        let mut verifier = sign::Verifier::new(MessageDigest::sha256(), self.0.as_ref())?;
        verifier.update(msg)?;

        verifier.verify(sig)
    }
}
