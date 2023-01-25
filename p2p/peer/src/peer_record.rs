use crate::peer_id::PeerID;

pub struct PeerRecord {
    peer_id: PeerID,
    seq: u64,
}
