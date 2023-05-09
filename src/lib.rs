#![cfg_attr(not(feature = "native"), no_std)]

use sovereign_sdk::da::DaSpec;

mod address;
mod block;
mod hash;
mod header;
#[cfg(feature = "native")]
mod service;
mod transaction;
mod types;
mod utils;
mod verifier;

pub struct DaLayerSpec;

impl DaSpec for DaLayerSpec {
    type SlotHash = hash::AvailHash;

    type Address = address::AvailAddress;

    type BlockHeader = header::AvailHeader;

    type BlobTransaction = transaction::AvailBlobTransaction;

    type InclusionMultiProof = ();

    type CompletenessProof = ();
}
