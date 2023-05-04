use sovereign_sdk::{
    da::BlobTransactionTrait,
    serial::{Decode, DecodeBorrowed, DeserializationError, Encode},
};

use crate::address::AvailAddress;

pub struct Data {}

impl sovereign_sdk::Buf for Data {
    fn remaining(&self) -> usize {
        todo!()
    }

    fn chunk(&self) -> &[u8] {
        todo!()
    }

    fn advance(&mut self, _cnt: usize) {}
}

pub struct AvailBlobTransaction {}

impl BlobTransactionTrait<AvailAddress> for AvailBlobTransaction {
    type Data = Data;

    fn sender(&self) -> AvailAddress {
        todo!()
    }

    fn data(&self) -> Self::Data {
        todo!()
    }
}

impl Decode for AvailBlobTransaction {
    type Error = DeserializationError;

    fn decode<R: std::io::Read>(_target: &mut R) -> Result<Self, <Self as Decode>::Error> {
        todo!()
    }
}

impl<'de> DecodeBorrowed<'de> for AvailBlobTransaction {
    type Error = DeserializationError;

    fn decode_from_slice(_target: &'de [u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Encode for AvailBlobTransaction {
    fn encode(&self, _target: &mut impl std::io::Write) {
        todo!()
    }
}
