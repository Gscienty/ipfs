mod ed25519;
mod rsa;
mod secp256k1;

use ed25519::*;
use rsa::*;
use secp256k1::*;

use crate::identity::PeerID;

pub enum PriKey {
    ED25519(Ed25519PriKey),
    RSA2048(RSAPriKey),
    SECP256K1(Secp256k1PriKey),
}

impl PriKey {
    pub fn generate(pritype: &str) -> Self {
        let pritype = multicodec::to_code(pritype);

        match pritype {
            0x1300 => Self::ED25519(Ed25519PriKey::generate().unwrap()),
            0x1301 => Self::SECP256K1(Secp256k1PriKey::generate().unwrap()),
            0x1305 => Self::RSA2048(RSAPriKey::generate(2048).unwrap()),

            _ => unreachable!(),
        }
    }

    pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
        match self {
            Self::ED25519(v) => v.sign(msg),
            Self::RSA2048(v) => v.sign(msg),
            Self::SECP256K1(v) => v.sign(msg),
        }
        .unwrap()
    }

    pub fn public_key(&self) -> PubKey {
        self.into()
    }
}

pub enum PubKey {
    ED25519(Ed25519PubKey),
    RSA2048(RSAPubKey),
    SECP256K1(Secp256k1PubKey),
}

impl From<&PriKey> for PubKey {
    fn from(key: &PriKey) -> Self {
        match key {
            PriKey::ED25519(ref v) => Self::ED25519(v.public_key().unwrap()),
            PriKey::RSA2048(ref v) => Self::RSA2048(v.public_key().unwrap()),
            PriKey::SECP256K1(ref v) => Self::SECP256K1(v.public_key().unwrap()),
        }
    }
}

impl PubKey {
    pub fn from_raw(pubtype: &str, pubkey: &[u8]) -> Self {
        let pubtype = multicodec::to_code(pubtype);

        match pubtype {
            0xed => Self::ED25519(Ed25519PubKey::from_raw(pubkey).unwrap()),
            0x1205 => Self::RSA2048(RSAPubKey::from_raw(pubkey).unwrap()),
            0xe7 => Self::SECP256K1(Secp256k1PubKey::from_raw(pubkey).unwrap()),

            _ => unreachable!(),
        }
    }

    pub fn verify(&self, msg: &[u8], sig: &[u8]) -> bool {
        let result = match self {
            Self::ED25519(v) => v.verify(msg, sig),
            Self::RSA2048(v) => v.verify(msg, sig),
            Self::SECP256K1(v) => v.verify(msg, sig),
        };

        if let Ok(result) = result {
            result
        } else {
            false
        }
    }

    pub fn raw_key(&self) -> Vec<u8> {
        match self {
            Self::ED25519(v) => v.raw_key(),
            Self::RSA2048(v) => v.raw_key(),
            Self::SECP256K1(v) => v.raw_key(),
        }
    }

    pub fn to_peer_id(&self) -> PeerID {
        self.into()
    }
}
