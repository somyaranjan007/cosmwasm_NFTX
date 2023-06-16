use cosmwasm_std::StdError;
use thiserror::Error;
use serde::{Serialize, Serializer};

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("User Doesn't Have Any Vault")]
    UserVaultNotFound {},

    #[error("Vault Not Found")]
    VaultNotFound {},

    #[error(" Nft Transfer Failed")]
    NftTransferFailed {},
}

impl Serialize for ContractError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        serializer.serialize_str("ContractError")   
    }
}