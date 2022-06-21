use cosmwasm_std::{ Uint128};
use schemars::{JsonSchema};
use serde::{Deserialize, Serialize};

use crate::state::AdminInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
   pub total_nft:Uint128,
   pub owner:String,
   pub check_mint:Vec<bool>,
   pub url:String,
   pub image_url:String,
   pub price:Uint128,
   pub denom:String,
   pub max_nft:Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Mint{rand:Uint128},
    SetNftAddress { address: String },
    SetAdmin {admin:Vec<AdminInfo>},
    ChangeOwner {address:String},
    ChangePrice {amount:Uint128}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
      GetStateInfo{},
      GetAdminInfo{},
      GetUserInfo{address:String}
    }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct JunoFarmingMsg {   
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Trait {
    pub trait_type: Option<String>,
    pub value: Option<String>,    
}