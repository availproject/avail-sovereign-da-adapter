use super::address::AvailAddress;
use crate::utils::deserialization_error;
use avail_subxt::{
    api::runtime_types::{da_control::pallet::Call, da_runtime::RuntimeCall::DataAvailability},
    primitives::AppUncheckedExtrinsic,
};
use codec::IoReader;
use sovereign_sdk::{
    da::BlobTransactionTrait,
    serial::{Decode, DecodeBorrowed, DeserializationError, Encode},
    Bytes,
};

#[derive(codec::Encode, codec::Decode, Clone, PartialEq, Debug)]
pub struct AvailBlobTransaction(pub AppUncheckedExtrinsic);

impl BlobTransactionTrait<AvailAddress> for AvailBlobTransaction {
    type Data = Bytes;

    fn sender(&self) -> AvailAddress {
        match &self.0.signature {
            Some((subxt::utils::MultiAddress::Id(id), _, _)) => AvailAddress(id.clone().0),
            _ => unimplemented!(),
        }
    }

    fn data(&self) -> Self::Data {
        match &self.0.function {
            DataAvailability(Call::submit_data { data }) => Bytes::copy_from_slice(&data.0),
            _ => unimplemented!(),
        }
    }
}

impl Decode for AvailBlobTransaction {
    type Error = DeserializationError;

    fn decode<R: std::io::Read>(target: &mut R) -> Result<Self, <Self as Decode>::Error> {
        codec::Decode::decode(&mut IoReader(target)).map_err(deserialization_error)
    }
}

impl<'de> DecodeBorrowed<'de> for AvailBlobTransaction {
    type Error = DeserializationError;

    fn decode_from_slice(mut target: &'de [u8]) -> Result<Self, Self::Error> {
        codec::Decode::decode(&mut target).map_err(deserialization_error)
    }
}

impl Encode for AvailBlobTransaction {
    fn encode(&self, target: &mut impl std::io::Write) {
        target
            .write_all(&codec::Encode::encode(&self))
            .expect("Serialization should not fail")
    }
}
