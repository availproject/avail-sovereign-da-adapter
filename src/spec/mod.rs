use sovereign_sdk::da::DaSpec;

mod address;
pub mod block;
mod hash;
pub mod header;
pub mod transaction;

pub struct DaLayerSpec;

impl DaSpec for DaLayerSpec {
    type SlotHash = hash::AvailHash;

    type Address = address::AvailAddress;

    type BlockHeader = header::AvailHeader;

    type BlobTransaction = transaction::AvailBlobTransaction;

    type InclusionMultiProof = ();

    type CompletenessProof = ();
}
