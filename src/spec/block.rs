use super::{header::AvailHeader, transaction::AvailBlobTransaction};
use serde::{Deserialize, Serialize};
use sovereign_sdk::{core::traits::CanonicalHash, services::da::SlotData};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct AvailBlock {
    pub header: AvailHeader,
    pub transactions: Vec<AvailBlobTransaction>,
}

impl SlotData for AvailBlock {
    fn hash(&self) -> [u8; 32] {
        self.header.hash().0 .0
    }
}
