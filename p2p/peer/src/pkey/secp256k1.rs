extern crate openssl;

use openssl::{ec, error::ErrorStack, hash::MessageDigest, nid, pkey, sign};

pub struct Secp256k1PriKey(pkey::PKey<pkey::Private>);

impl Secp256k1PriKey {
    pub(crate) fn generate() -> Result<Self, ErrorStack> {
        let ecgroup = ec::EcGroup::from_curve_name(nid::Nid::SECP256K1)?;
        let private_key = ec::EcKey::generate(&ecgroup)?;

        Ok(Self(pkey::PKey::from_ec_key(private_key)?))
    }

    pub(crate) fn from_raw(raw: &[u8]) -> Result<Self, ErrorStack> {
        let private_key = pkey::PKey::private_key_from_raw_bytes(raw, pkey::Id::EC)?;

        Ok(Self(private_key))
    }

    pub(crate) fn public_key(&self) -> Result<Secp256k1PubKey, ErrorStack> {
        let raw = self.0.raw_public_key()?;

        Secp256k1PubKey::from_raw(&raw)
    }

    pub(crate) fn raw_key(&self) -> Vec<u8> {
        self.0.raw_private_key().unwrap()
    }

    pub(crate) fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, ErrorStack> {
        let mut signer = sign::Signer::new(MessageDigest::sha256(), &self.0.as_ref())?;
        signer.update(msg)?;

        signer.sign_to_vec()
    }
}

pub struct Secp256k1PubKey(pkey::PKey<pkey::Public>);

impl Secp256k1PubKey {
    pub(crate) fn from_raw(raw: &[u8]) -> Result<Self, ErrorStack> {
        Ok(Self(pkey::PKey::public_key_from_raw_bytes(
            raw,
            pkey::Id::EC,
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
