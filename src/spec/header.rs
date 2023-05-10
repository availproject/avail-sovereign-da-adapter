use super::hash::AvailHash;
use avail_subxt::primitives::Header;
use sovereign_sdk::core::traits::{BlockHeaderTrait, CanonicalHash};
use subxt::utils::H256;

#[derive(codec::Encode, codec::Decode, Debug, PartialEq, Clone)]
pub struct AvailHeader {
    #[codec(skip)]
    hash: AvailHash,

    #[codec(skip)]
    pub prev_hash: AvailHash,

    pub header: Header,
}

impl AvailHeader {
    pub fn new(header: Header, hash: H256) -> Self {
        Self {
            hash: AvailHash(hash),
            prev_hash: AvailHash(header.parent_hash),
            header,
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
