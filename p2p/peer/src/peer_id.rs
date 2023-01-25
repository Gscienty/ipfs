use multibase::Encoding;
use multihash::multihash;

use crate::pkey;

pub struct PeerID {
    hash: Vec<u8>,
}

impl From<&pkey::PubKey> for PeerID {
    fn from(prikey: &pkey::PubKey) -> Self {
        let hash = multihash("sha2-256", &prikey.raw_key());

        PeerID { hash }
    }
}

impl From<&PeerID> for String {
    fn from(peer_id: &PeerID) -> Self {
        multibase::Base58Bitcoin::encode(&peer_id.hash)
    }
}
