use std::str::FromStr;

use cid::Cid;
use fvm_shared::address::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
    pub installed_actor_cid: Cid,
    pub deployed_actor_id: Address,
    pub deployed_actor_robust: Address,
}

impl State {
    pub fn new() -> anyhow::Result<State> {
        Ok(State {
            installed_actor_cid: Cid::from_str(
                "bafk2bzaceasfvd45kxtyn7kktsmo3bpcgbxm5nqqpqon22e7huoinhhdmkh6q",
            )
            .unwrap(),
            deployed_actor_id: Address::new_id(0),
            deployed_actor_robust: Address::new_id(0),
        })
    }
}
