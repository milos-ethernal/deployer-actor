use fvm_shared::address::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
    pub deployed_actor_id: Address,
    pub deployed_actor_robust: Address,
}

impl State {
    pub fn new() -> anyhow::Result<State> {
        Ok(State {
            deployed_actor_id: Address::new_id(0),
            deployed_actor_robust: Address::new_id(0),
        })
    }
}
