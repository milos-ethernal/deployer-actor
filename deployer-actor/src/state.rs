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
            installed_actor_cid: Cid::default(),
            deployed_actor_id: Address::new_id(0),
            deployed_actor_robust: Address::new_id(0),
        })
    }
}
