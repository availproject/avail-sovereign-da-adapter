use crate::spec::DaLayerSpec;
use core::convert::Infallible;
use sovereign_sdk::da::{DaSpec, DaVerifier};

pub struct Verifier;

impl DaVerifier for Verifier {
    type Spec = DaLayerSpec;

    type Error = Infallible;

    // Verify that the given list of blob transactions is complete and correct.
    // NOTE: Function is unimplemented since application client already verifies application data.
    fn verify_relevant_tx_list(
        &self,
        _block_header: &<Self::Spec as DaSpec>::BlockHeader,
        _txs: &[<Self::Spec as DaSpec>::BlobTransaction],
        _inclusion_proof: <Self::Spec as DaSpec>::InclusionMultiProof,
        _completeness_proof: <Self::Spec as DaSpec>::CompletenessProof,
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn new(_params: <Self::Spec as DaSpec>::ChainParams) -> Self {
        Verifier {}
    }
}
