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
        "YOUR_PRIVAE_KEY_bs58",
        "https://api.devnet.solana.com",
        "openai_api_key",
    ));

    let res = agent
        .launch_token_pumpfun(
            "Name",
            "Symbol",
            "this is a description.",
            "https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png",
            None,
        )
        .await
        .unwrap();

    println!("Pumpfun Token response: {:?}", res);
}
