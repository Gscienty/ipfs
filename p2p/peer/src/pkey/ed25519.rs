extern crate openssl;

use openssl::{error::ErrorStack, pkey, sign};

pub struct Ed25519PriKey(pkey::PKey<pkey::Private>);

impl Ed25519PriKey {
    pub(crate) fn generate() -> Result<Self, ErrorStack> {
        Ok(Self(pkey::PKey::generate_ed25519()?))
    }

    pub(crate) fn from_raw(raw: &[u8]) -> Result<Self, ErrorStack> {
        let private_key = pkey::PKey::private_key_from_raw_bytes(raw, pkey::Id::ED25519)?;

        Ok(Self(private_key))
    }

    pub(crate) fn public_key(&self) -> Result<Ed25519PubKey, ErrorStack> {
        let raw = self.0.raw_public_key()?;

        Ed25519PubKey::from_raw(&raw)
    }

    pub(crate) fn raw_key(&self) -> Vec<u8> {
        self.0.raw_private_key().unwrap()
    }

    pub(crate) fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, ErrorStack> {
        let mut signer = sign::Signer::new_without_digest(self.0.as_ref())?;

        signer.sign_oneshot_to_vec(msg)
    }
}

pub struct Ed25519PubKey(pkey::PKey<pkey::Public>);

impl Ed25519PubKey {
    pub(crate) fn from_raw(raw: &[u8]) -> Result<Self, ErrorStack> {
        Ok(Self(pkey::PKey::public_key_from_raw_bytes(
            raw,
            pkey::Id::ED25519,
        )?))
    }

    pub(crate) fn raw_key(&self) -> Vec<u8> {
        self.0.raw_public_key().unwrap()
    }

    pub(crate) fn verify(&self, msg: &[u8], sig: &[u8]) -> Result<bool, ErrorStack> {
        let mut verifier = sign::Verifier::new_without_digest(self.0.as_ref())?;

        verifier.verify_oneshot(sig, msg)
    }
}
