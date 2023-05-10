use super::{header::AvailHeader, transaction::AvailBlobTransaction};
use crate::utils::deserialization_error;
use codec::IoReader;
use sovereign_sdk::{
    core::traits::CanonicalHash,
    serial::{Decode, DecodeBorrowed, DeserializationError, Encode},
    services::da::SlotData,
};

#[derive(codec::Encode, codec::Decode, PartialEq, Clone, Debug)]
pub struct AvailBlock {
    pub header: AvailHeader,
    pub transactions: Vec<AvailBlobTransaction>,
}

impl SlotData for AvailBlock {
    fn hash(&self) -> [u8; 32] {
        self.header.hash().0 .0
    }
}

impl Decode for AvailBlock {
    type Error = DeserializationError;

    fn decode<R: std::io::Read>(target: &mut R) -> Result<Self, <Self as Decode>::Error> {
        codec::Decode::decode(&mut IoReader(target)).map_err(deserialization_error)
    }
}

impl<'de> DecodeBorrowed<'de> for AvailBlock {
    type Error = DeserializationError;

    fn decode_from_slice(mut target: &'de [u8]) -> Result<Self, Self::Error> {
        codec::Decode::decode(&mut target).map_err(deserialization_error)
    }
}

impl Encode for AvailBlock {
    fn encode(&self, target: &mut impl std::io::Write) {
        target
            .write_all(&codec::Encode::encode(&self))
            .expect("Serialization should not fail")
    }
}
