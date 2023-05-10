use borsh::{BorshDeserialize, BorshSerialize};
use sovereign_sdk::core::traits::AddressTrait;

#[derive(Debug, PartialEq, Clone, Eq, BorshSerialize, BorshDeserialize)]
pub struct AvailAddress(pub [u8; 32]);

impl AddressTrait for AvailAddress {}

impl AsRef<[u8]> for AvailAddress {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<'a> TryFrom<&'a [u8]> for AvailAddress {
    type Error = anyhow::Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self(<[u8; 32]>::try_from(value)?))
    }
}
