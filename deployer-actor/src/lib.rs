include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod deployer_actor;
pub mod state;
pub mod types;
