use crate::utils::data_to_short;
use sovereign_sdk::{
    da::BlockHashTrait,
    serial::{Decode, DecodeBorrowed, DeserializationError, Encode},
};
use subxt::utils::H256;

#[derive(Clone, Debug, PartialEq)]
pub struct AvailHash(pub H256);

impl BlockHashTrait for AvailHash {}

impl Decode for AvailHash {
    type Error = DeserializationError;

    fn decode<R: std::io::Read>(target: &mut R) -> Result<Self, <Self as Decode>::Error> {
        let mut out = [0u8; 32];
        target
            .read_exact(&mut out)
            .map_err(|_| data_to_short(32, 0))?;
        Ok(AvailHash(H256::from_slice(&out)))
    }
}

impl<'de> DecodeBorrowed<'de> for AvailHash {
    type Error = DeserializationError;

    fn decode_from_slice(target: &'de [u8]) -> Result<Self, Self::Error> {
        if target.len() < 32 {
            return Err(data_to_short(32, target.len()));
        }
        Ok(AvailHash(H256::from_slice(&target[..32])))
    }
}

impl Encode for AvailHash {
    fn encode(&self, target: &mut impl std::io::Write) {
        target
            .write_all(self.0.as_ref())
            .expect("Serialization should not fail")
    }
}

impl AsRef<[u8]> for AvailHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use sovereign_sdk::serial::{Decode, Encode};
    use subxt::utils::H256;

    use crate::hash::AvailHash;

    #[test]
    fn test_hash() {
        let avail_hash = AvailHash(H256::random());

        let mut out = Vec::new();
        avail_hash.encode(&mut out);

        assert_eq!(avail_hash, Decode::decode(&mut out.as_slice()).unwrap());
    }
}
