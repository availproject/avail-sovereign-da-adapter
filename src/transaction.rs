use avail_subxt::{
    api::runtime_types::{da_control::pallet::Call, da_runtime::RuntimeCall::DataAvailability},
    primitives::AppUncheckedExtrinsic,
};
use sovereign_sdk::{
    da::BlobTransactionTrait,
    serial::{Decode, DecodeBorrowed, DeserializationError, Encode},
    Bytes,
};

use crate::{address::AvailAddress, utils::data_to_short};

#[derive(codec::Encode, codec::Decode, Clone, PartialEq, Debug)]
pub struct AvailBlobTransaction(pub AppUncheckedExtrinsic);

impl BlobTransactionTrait<AvailAddress> for AvailBlobTransaction {
    type Data = Bytes;

    fn sender(&self) -> AvailAddress {
        match &self.0.signature {
            Some((subxt::utils::MultiAddress::Id(id), _, _)) => AvailAddress(id.clone().0),
            _ => todo!(),
        }
    }

    fn data(&self) -> Self::Data {
        match &self.0.function {
            DataAvailability(Call::submit_data { data }) => Bytes::copy_from_slice(&data.0),
            _ => todo!(),
        }
    }
}

impl Decode for AvailBlobTransaction {
    type Error = DeserializationError;

    fn decode<R: std::io::Read>(target: &mut R) -> Result<Self, <Self as Decode>::Error> {
        let mut out = vec![];
        target
            .read_to_end(&mut out)
            .map_err(|_| data_to_short(0, 0))?;
        DecodeBorrowed::decode_from_slice(&out)
    }
}

impl<'de> DecodeBorrowed<'de> for AvailBlobTransaction {
    type Error = DeserializationError;

    fn decode_from_slice(mut target: &'de [u8]) -> Result<Self, Self::Error> {
        codec::Decode::decode(&mut target).map_err(|_error| data_to_short(0, 0))
    }
}

impl Encode for AvailBlobTransaction {
    fn encode(&self, target: &mut impl std::io::Write) {
        let encoded = codec::Encode::encode(&self);
        target
            .write_all(&encoded)
            .expect("Serialization should not fail")
    }
}
