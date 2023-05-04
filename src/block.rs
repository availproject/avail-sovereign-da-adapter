use crate::{header::AvailHeader, utils::data_to_short};
use avail_subxt::primitives::Header;
use codec::Decode as CodecDecode;
use sovereign_sdk::{
    core::traits::CanonicalHash,
    serial::{Decode, DecodeBorrowed, DeserializationError, Encode},
    services::da::SlotData,
};

#[derive(PartialEq, Clone, Debug)]
pub struct AvailBlock {
    pub header: AvailHeader,
}

impl SlotData for AvailBlock {
    fn hash(&self) -> [u8; 32] {
        self.header.hash().0 .0
    }
}

impl Decode for AvailBlock {
    type Error = DeserializationError;

    fn decode<R: std::io::Read>(target: &mut R) -> Result<Self, <Self as Decode>::Error> {
        let mut out = vec![];
        target
            .read_to_end(&mut out)
            .map_err(|_| data_to_short(0, 0))?;
        DecodeBorrowed::decode_from_slice(&out)
    }
}

impl<'de> DecodeBorrowed<'de> for AvailBlock {
    type Error = DeserializationError;

    fn decode_from_slice(mut target: &'de [u8]) -> Result<Self, Self::Error> {
        let header = Header::decode(&mut target).map_err(|_error| data_to_short(0, 0))?;
        Ok(AvailBlock {
            header: AvailHeader::new(header),
        })
    }
}

impl Encode for AvailBlock {
    fn encode(&self, target: &mut impl std::io::Write) {
        let encoded = codec::Encode::encode(&self.header.header);
        target
            .write_all(&encoded)
            .expect("Serialization should not fail")
    }
}