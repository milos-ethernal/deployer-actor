use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {}

impl State {
    pub fn new() -> anyhow::Result<State> {
        Ok(State {})
    }
}
