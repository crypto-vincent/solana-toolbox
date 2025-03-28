use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

use crate::toolbox_endpoint::ToolboxEndpoint;
use crate::toolbox_endpoint_proxy::ToolboxEndpointProxy;
use crate::toolbox_endpoint_proxy_rpc_client::ToolboxEndpointProxyRpcClient;

impl ToolboxEndpoint {
    pub fn get_url_from_url_or_moniker(url_or_moniker: &str) -> &str {
        match url_or_moniker {
            "m" | "mainnet-beta" => "https://api.mainnet-beta.solana.com",
            "t" | "testnet" => "https://api.testnet.solana.com",
            "d" | "devnet" => "https://api.devnet.solana.com",
            "l" | "localhost" => "http://localhost:8899",
            url => url,
        }
    }

    pub fn get_cluster_from_url_or_moniker(
        url_or_moniker: &str,
    ) -> Option<&str> {
        match url_or_moniker {
            "m" | "mainnet-beta" | "https://api.mainnet-beta.solana.com" => {
                Some("mainnet-beta")
            },
            "t" | "testnet" | "https://api.testnet.solana.com" => {
                Some("testnet")
            },
            "d" | "devnet" | "https://api.devnet.solana.com" => Some("devnet"),
            _ => None,
        }
    }

    pub fn new_rpc_with_url_or_moniker_and_commitment(
        url_or_moniker: &str,
        commitment_config: CommitmentConfig,
    ) -> ToolboxEndpoint {
        let url = ToolboxEndpoint::get_url_from_url_or_moniker(url_or_moniker);
        RpcClient::new_with_commitment(url.to_string(), commitment_config)
            .into()
    }
}

impl From<RpcClient> for ToolboxEndpoint {
    fn from(rpc_client: RpcClient) -> Self {
        let proxy: Box<dyn ToolboxEndpointProxy> =
            Box::new(ToolboxEndpointProxyRpcClient::new(rpc_client));
        ToolboxEndpoint::from(proxy)
    }
}
