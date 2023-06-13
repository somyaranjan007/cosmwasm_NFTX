use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::Vault;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateVault { nft_asset_address: Addr, vault_name: String, vault_symbol: String }, 
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetVaultResponse)]
    GetVault { vault_owner: Addr }, 

    #[returns(GetVaultArrayResponse)]
    GetVaultArray {},
}

#[cw_serde]
pub struct GetVaultResponse {
    pub vault_response: Vec<Vault>
}

#[cw_serde]
pub struct GetVaultArrayResponse {
    pub vault_array: Vec<Vault>
}