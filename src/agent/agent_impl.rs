// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    actions::{
        deploy_collection, deploy_token, fetch_price, fetch_price_by_pyth,
        fetch_pyth_price_feed_id, get_balance, get_balance_other, get_tps, launch_token_pumpfun,
        mint_nft_to_collection, request_faucet_funds, transfer, swap, stake_sol
    },
    agent::SolAgent,
    primitives::{
        pumpfun::{PumpFunTokenOptions, PumpfunTokenResponse},
        token::{DeployedData, NftMetadata},
    },
};
use solana_client::client_error::ClientError;
use solana_sdk::pubkey::Pubkey;

impl SolAgent {
    pub async fn get_balance(&self, token_address: Option<String>) -> Result<f64, ClientError> {
        get_balance(&self, token_address).await
    }

    pub async fn request_faucet_funds(&self) -> Result<String, ClientError> {
        request_faucet_funds(&self).await
    }

    pub async fn get_balance_other(
        &self,
        wallet_address: Pubkey,
        token_address: Option<Pubkey>,
    ) -> Result<f64, ClientError> {
        get_balance_other(&self, wallet_address, token_address).await
    }

    pub async fn get_tps(&self) -> Result<f64, ClientError> {
        get_tps(&self).await
    }

    pub async fn deploy_token(
        &self,
        name: String,
        uri: String,
        symbol: String,
        decimals: u8,
        initial_supply: Option<u64>,
    ) -> Result<Pubkey, ClientError> {
        deploy_token(&self, name, uri, symbol, decimals, initial_supply).await
    }

    pub async fn deploy_collection(
        &self,
        metadata: NftMetadata,
    ) -> Result<(String, String), ClientError> {
        deploy_collection(&self, &metadata).await
    }

    pub async fn mint_nft_to_collection(
        &self,
        collection: Pubkey,
        metadata: NftMetadata,
    ) -> Result<DeployedData, ClientError> {
        mint_nft_to_collection(&self, collection, metadata).await
    }

    pub async fn fetch_price(token_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        fetch_price(token_id).await
    }

    pub async fn fetch_price_by_pyth(
        price_feed_id: &str,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        fetch_price_by_pyth(price_feed_id).await
    }

    pub async fn fetch_pyth_price_feed_id(
        token_symbol: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        fetch_pyth_price_feed_id(token_symbol).await
    }

    pub async fn transfer(
        &self,
        to: &str,
        amount: u64,
        mint: Option<String>,
    ) -> Result<String, ClientError> {
        transfer(&self, to, amount, mint).await
    }

    pub async fn launch_token_pumpfun(
        &self,
        token_name: &str,
        token_ticker: &str,
        description: &str,
        image_url: &str,
        options: Option<PumpFunTokenOptions>,
    ) -> Result<PumpfunTokenResponse, Box<dyn std::error::Error>> {
        launch_token_pumpfun(
            &self,
            token_name,
            token_ticker,
            description,
            image_url,
            options,
        )
        .await
    }

    pub async fn jupiter_swap(
        &self,
        from_token: Option<&str>,
        amount: f64,
        to_token: &str,
        slippage: Option<u32>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        swap(&self, to_token, amount, from_token, slippage).await
    }

    pub async fn jupiter_stake_sol(
        &self,
        amount: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        stake_sol(&self, amount).await
    }

}
