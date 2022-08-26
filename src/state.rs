use cw_storage_plus::{Item};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const CONFIG: Item<State> = Item::new("config_state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
   pub admin: String,
   pub denom: String,
   pub pubkey:PubKey
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PubKey {
   pub n: Vec<u8>,
   pub e: Vec<u8>
}

