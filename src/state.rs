use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cw20Deposits {
    pub count: i32,
    pub owner: String,
    pub contract:String,
    pub amount:u128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cw721Deposits {
    pub owner: String,
    pub contract:String,
    pub token_id:String
}

//key is address, denom
pub const CW20_DEPOSITS: Map<(&str, &str), Cw20Deposits> = Map::new("cw20deposits");

//contract, owner, token_id
pub const CW721_DEPOSITS: Map<(&str, &str, &str), Cw721Deposits> = Map::new("cw721deposits");
