use cosmwasm_std::Uint128;
use cw_storage_plus::{Map,Item};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::msg::{JunoPunksMsg};

pub const CONFIG: Item<State> = Item::new("config_state");
pub const METADATA: Item<Vec<JunoPunksMsg>> = Item::new("metadata");

pub const ADMININFO: Item< Vec<AdminInfo>> = Item::new("offerings");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub total_nft:Uint128,
    pub owner:String,
    pub count : Uint128,
    pub check_mint:Vec<bool>,
    pub nft_address:String,
    pub url :String,
    pub image_url:String,
    pub price:Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AdminInfo {
    pub address:String,
    pub amount:Uint128
}

