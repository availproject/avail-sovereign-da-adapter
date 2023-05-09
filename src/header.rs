use crate::hash::AvailHash;
use avail_subxt::primitives::Header;
use sovereign_sdk::core::traits::{BlockHeaderTrait, CanonicalHash};
use sp_core::{blake2_256, Encode};

#[derive(codec::Encode, codec::Decode, Debug, PartialEq, Clone)]
pub struct AvailHeader {
    pub header: Header,

    #[codec(skip)]
    hash: AvailHash,

    #[codec(skip)]
    pub prev_hash: AvailHash,
}

impl AvailHeader {
    pub fn new(header: Header) -> Self {
        let hash = AvailHash(Encode::using_encoded(&header, blake2_256).into());
        let prev_hash = AvailHash(header.parent_hash);
        Self {
            header,
            hash,
            prev_hash,
        }
    }
}

impl BlockHeaderTrait for AvailHeader {
    type Hash = AvailHash;

    fn prev_hash(&self) -> &Self::Hash {
        &self.prev_hash
    }
}

impl CanonicalHash for AvailHeader {
    type Output = AvailHash;

    fn hash(&self) -> Self::Output {
        self.hash.clone()
    }
}
