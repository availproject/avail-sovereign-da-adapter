use crate::{
    avail::{Confidence, ExtrinsicsData},
    spec::{
        block::AvailBlock, header::AvailHeader, transaction::AvailBlobTransaction, DaLayerSpec,
    },
};
use avail_subxt::AvailConfig;
use core::{future::Future, pin::Pin};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sovereign_sdk::{da::DaSpec, services::da::DaService};
use subxt::OnlineClient;
use tracing::info;

pub struct DaProvider {
    pub node_client: OnlineClient<AvailConfig>,
    pub light_client_url: String,
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

#[derive(Serialize, Deserialize)]
pub struct RuntimeConfig {
    light_client_url: String,
    #[serde(skip)]
    node_client: Option<OnlineClient<AvailConfig>>,
}

impl DaService for DaProvider {
    type RuntimeConfig = RuntimeConfig;

    type Spec = DaLayerSpec;

    type FilteredBlock = AvailBlock;

    type Future<T> = Pin<Box<dyn Future<Output = Result<T, Self::Error>>>>;

    type Error = anyhow::Error;

    // Make an RPC call to the node to get the finalized block at the given height, if one exists.
    // If no such block exists, block until one does.
    fn get_finalized_at(&self, height: u64) -> Self::Future<Self::FilteredBlock> {
        let node_client = self.node_client.clone();
        let confidence_url = self.confidence_url(height);
        let appdata_url = self.appdata_url(height);

        Box::pin(async move {
            // NOTE: Only supported case is when application data is present and verified
            let response = reqwest::get(confidence_url).await?;
            if response.status() != StatusCode::OK {
                unimplemented!()
            }
            let response: Confidence = serde_json::from_str(&response.text().await?)?;
            if response.confidence < 92.5 {
                unimplemented!()
            }
            let response = reqwest::get(appdata_url).await?;
            if response.status() != StatusCode::OK {
                unimplemented!()
            }
            let appdata: ExtrinsicsData = serde_json::from_str(&response.text().await?)?;

            info!("Appdata: {:?}", appdata);

            let hash = node_client
                .rpc()
                .block_hash(Some(height.into()))
                .await?
                .unwrap();

            info!("Hash: {:?}", hash);

            let header = node_client.rpc().header(Some(hash)).await?.unwrap();

            info!("Header: {:?}", header);

            let header = AvailHeader::new(header, hash);
            let transactions = appdata
                .extrinsics
                .into_iter()
                .map(AvailBlobTransaction)
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
    ) -> Vec<<Self::Spec as DaSpec>::BlobTransaction> {
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
        Vec<<Self::Spec as DaSpec>::BlobTransaction>,
        <Self::Spec as DaSpec>::InclusionMultiProof,
        <Self::Spec as DaSpec>::CompletenessProof,
    ) {
        (block.transactions, (), ())
    }

    fn new(
        config: Self::RuntimeConfig,
        _chain_params: <Self::Spec as DaSpec>::ChainParams,
    ) -> Self {
        let node_client = config.node_client.unwrap();
        let light_client_url = config.light_client_url;

        DaProvider {
            node_client,
            light_client_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::service::RuntimeConfig;

    use super::DaProvider;
    use avail_subxt::build_client;
    use sovereign_sdk::services::da::DaService;

    #[tokio::test]
    async fn get_finalized_at() {
        tracing_subscriber::fmt::init();

        let node_ws = "ws://127.0.0.1:9944";
        let light_client_url = "http://127.0.0.1:7000".to_string();
        let node_client = Some(build_client(node_ws).await.unwrap());
        let runtime_config = RuntimeConfig {
            node_client,
            light_client_url,
        };
        let da_service = DaProvider::new(runtime_config, ());
        da_service.get_finalized_at(1).await.unwrap();
    }
}
