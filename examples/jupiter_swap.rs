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

use solagent::SolAgent;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let agent = Arc::new(SolAgent::new(
        "private_key",
        "https://api.devnet.solana.com",
        "openai_api_key",
    ));
    //swap 0.01 SOL to USDC
    let swap = agent.jupiter_swap(Some("So11111111111111111111111111111111111111112"), 0.01, "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", None).await.unwrap();
    println!("Signature: {}", swap);
}
