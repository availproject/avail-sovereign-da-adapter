use core::convert::Infallible;

use sovereign_sdk::da::DaVerifier;

use crate::DaLayerSpec;

pub struct Verifier;

impl DaVerifier for Verifier {
    type Spec = DaLayerSpec;

    type Error = Infallible;

    // Verify that the given list of blob transactions is complete and correct.
    fn verify_relevant_tx_list(
        &self,
        _block_header: &<Self::Spec as sovereign_sdk::da::DaSpec>::BlockHeader,
        _txs: &[<Self::Spec as sovereign_sdk::da::DaSpec>::BlobTransaction],
        _inclusion_proof: <Self::Spec as sovereign_sdk::da::DaSpec>::InclusionMultiProof,
        _completeness_proof: <Self::Spec as sovereign_sdk::da::DaSpec>::CompletenessProof,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
