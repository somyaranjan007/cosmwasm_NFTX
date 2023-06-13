use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{ Item, Map };

#[cw_serde]
pub struct State {
    pub contract_owner: Addr,
}

#[cw_serde]
pub struct Vault {
    pub vault_id: u64,
    pub vault_name: String,
    pub vault_symbol: String,
    pub nft_asset_address: Addr,
}

pub const STATE: Item<State> = Item::new("contract_owner");

pub const USER_VAULTS: Map<&str, Vec<u64>> = Map::new("user_vaults");
pub const VAULT_LIST: Item<Vec<Vault>> = Item::new("vault_list");