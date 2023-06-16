use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr,Uint128};


use crate::state::Vault;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateVault {
        nft_asset_address: Addr,
        vault_name: String,
        vault_symbol: String,
    },
    ManageNft{
        recipient:Addr ,
         nft_id:u64,
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetVaultIdResponse)]
    GetVaultId { vault_owner: Addr },

    #[returns(GetVaultArrayResponse)]
    GetVaultArray {},
}

#[cw_serde]
pub struct GetVaultIdResponse {
    pub vault_id_response: Vec<u64>,
}

#[cw_serde]
pub struct GetVaultArrayResponse {
    pub vault_array: Vec<Vault>,
}

#[cw_serde]
pub struct VtokenInstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub nft_id: u64,
}

#[cw_serde]
pub enum VtokenExecuteMsg {
    Mint {
        account: Addr,
        amount: Uint128,
    },
    Burn {
        amount: Uint128,
    },
    Transfer {

        to_account: Addr,
        amount: Uint128,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum VtokenQueryResponses {}
