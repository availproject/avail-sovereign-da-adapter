use crate::{
    block::AvailBlock,
    header::AvailHeader,
    transaction::AvailBlobTransaction,
    types::{Confidence, ExtrinsicsData},
    DaLayerSpec,
};
use avail_subxt::AvailConfig;
use core::{future::Future, pin::Pin};
use reqwest::StatusCode;
use sovereign_sdk::services::da::DaService;
use subxt::OnlineClient;
use tracing::info;

pub struct DaProvider {
    node_client: OnlineClient<AvailConfig>,
    light_client_url: String,
}

impl DaProvider {
    fn appdata_url(&self, block_num: u64) -> String {
        let light_client_url = self.light_client_url.clone();
        format!("{light_client_url}/v1/appdata/{block_num}")
    }

    fn confidence_url(&self, block_num: u64) -> String {
        let light_client_url = self.light_client_url.clone();
        format!("{light_client_url}/v1/confidence/{block_num}")
    }
}

impl DaService for DaProvider {
    type Spec = DaLayerSpec;

    type FilteredBlock = AvailBlock;

    type Future<T> = Pin<Box<dyn Future<Output = Result<T, Self::Error>>>>;

    type Error = anyhow::Error;

    // Make an RPC call to the node to get the finalized block at the given height, if one exists.
    // If no such block exists, block until one does.
    fn get_finalized_at(&self, height: u64) -> Self::Future<Self::FilteredBlock> {
        let client = self.node_client.clone();
        let confidence_url = self.confidence_url(height);
        let appdata_url = self.appdata_url(height);
        Box::pin(async move {
            let response = reqwest::get(confidence_url).await?;
            if response.status() != StatusCode::OK {
                todo!()
            }
            let response: Confidence = serde_json::from_str(&response.text().await?)?;
            if response.confidence < 92.5 {
                todo!()
            }
            let response = reqwest::get(appdata_url).await?;
            if response.status() != StatusCode::OK {
                todo!()
            }
            let appdata: ExtrinsicsData = serde_json::from_str(&response.text().await?)?;
            info!("Appdata: {:?}", appdata);
            let hash = client.rpc().block_hash(Some(height.into())).await?.unwrap();
            info!("Hash: {:?}", hash);
            let header = client.rpc().header(Some(hash)).await?.unwrap();
            info!("Header: {:?}", header);
            let header = AvailHeader::new(header);
            let transactions = appdata
                .extrinsics
                .into_iter()
                .map(|e| AvailBlobTransaction(e))
                .collect();
            Ok(AvailBlock {
                header,
                transactions,
            })
        })
    }

    // Make an RPC call to the node to get the block at the given height
    // If no such block exists, block until one does.
    fn get_block_at(&self, height: u64) -> Self::Future<Self::FilteredBlock> {
        self.get_finalized_at(height)
    }

    // Extract the blob transactions relevant to a particular rollup from a block.
    // This method is usually (but not always) parameterized by some configuration option,
    // such as the rollup's namespace on Celestia. If configuration is needed, it should be provided
    // to the DaProvider struct through its constructor.
    fn extract_relevant_txs(
        &self,
        block: Self::FilteredBlock,
    ) -> Vec<<Self::Spec as sovereign_sdk::da::DaSpec>::BlobTransaction> {
        block.transactions
    }

    // Extract the list blob transactions relevant to a particular rollup from a block, along with inclusion and
    // completeness proofs for that set of transactions. The output of this method will be passed to the verifier.
    //
    // Like extract_relevant_txs, This method is usually (but not always) parameterized by some configuration option,
    // such as the rollup's namespace on Celestia. If configuration is needed, it should be provided
    // to the DaProvider struct through its constructor.
    fn extract_relevant_txs_with_proof(
        &self,
        block: Self::FilteredBlock,
    ) -> (
        Vec<<Self::Spec as sovereign_sdk::da::DaSpec>::BlobTransaction>,
        <Self::Spec as sovereign_sdk::da::DaSpec>::InclusionMultiProof,
        <Self::Spec as sovereign_sdk::da::DaSpec>::CompletenessProof,
    ) {
        (block.transactions, (), ())
    }
}

#[cfg(test)]
mod tests {
    use super::DaProvider;
    use avail_subxt::build_client;
    use sovereign_sdk::services::da::DaService;

    #[tokio::test]
    async fn get_finalized_at() {
        tracing_subscriber::fmt::init();

        let node_ws = "ws://127.0.0.1:9944";
        let light_client_url = "http://127.0.0.1:7000".to_string();
        let node_client = build_client(node_ws).await.unwrap();
        let da_service = DaProvider {
            node_client,
            light_client_url,
        };
        da_service.get_finalized_at(357).await.unwrap();
    }
}
