#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr};
use cw2::set_contract_version; 

use cw721_base::state::Cw721Contract;
use cw721_base::{Extension};
use crate::state::{ State, STATE, Vault, USER_VAULTS, VAULT_LIST };
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, GetVaultIdResponse, GetVaultArrayResponse};
use crate::msg::{VtokenExecuteMsg,VtokenInstantiateMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmwasm-nftx";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    let set_owner = State {
        contract_owner: _info.sender,
    };

    STATE.save(_deps.storage, &set_owner)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::CreateVault { nft_asset_address, vault_name, vault_symbol } => execute::create_vault(_deps, _env, _info, nft_asset_address, vault_name, vault_symbol),
        ExecuteMsg::ManageNft { recipient,nft_id }=> execute::ManageNft(_deps, _env, _info, recipient.to_string(), nft_id)
    }
}

pub mod execute {
    use cosmwasm_std::{Uint128, Empty};

    use crate::msg;

    use super::*;
    
    pub fn create_vault(_deps: DepsMut, _env: Env, _info: MessageInfo, _nft_asset_address: Addr, _vault_name: String, _vault_symbol: String) -> Result<Response, ContractError> {

        let is_vault_available = VAULT_LIST.load(_deps.storage);
        let mut _vault_id: u64;

        match is_vault_available {
            Ok(value) => {
                _vault_id = value.len() as u64 + 1;
            },
            Err(_) => {
                _vault_id = 1;
            } 
        }

        let vault = Vault {
            vault_id: _vault_id,
            vault_name: _vault_name,
            vault_symbol: _vault_symbol,
            nft_asset_address: _nft_asset_address
        };

        USER_VAULTS.update(_deps.storage, _info.sender.to_string(), |vault_array| -> Result<Vec<u64>, ContractError> {
            match vault_array {
                Some(mut vec_arr) => {
                    vec_arr.push(_vault_id);
                    Ok(vec_arr)
                },
                None => {
                    let mut new_vault_id_array: Vec<u64> = Vec::new();
                    new_vault_id_array.push(_vault_id);
                    Ok(new_vault_id_array)
                }
            }
        })?;

        let is_empty_vault_list = VAULT_LIST.may_load(_deps.storage)?;

        match is_empty_vault_list {
            Some(_) => {
                VAULT_LIST.update(_deps.storage, |mut vault_array| -> StdResult<_> {
                    vault_array.push(vault);
                    Ok(vault_array)
                })?;
            },
            None => {
                let mut new_vault_array: Vec<Vault> = Vec::new();
                new_vault_array.push(vault);
                VAULT_LIST.save(_deps.storage, &new_vault_array)?;
            }
        }

        Ok(Response::new().add_attribute("method", "create_vault"))
    }

    pub fn ManageNft(_deps: DepsMut, _env: Env, _info: MessageInfo,recipient:String , nft_token_id:u64)->Result<Response, ContractError>{
        

        let contract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();

        let transfer_nft = contract._transfer_nft(_deps, &_env, &_info, &recipient, &nft_token_id.to_string());

        match transfer_nft{
            Ok(_)=>{
                let msg = VtokenInstantiateMsg{name:"s".to_string() ,symbol:"s".to_string() , nft_id:nft_token_id};
            },
            Err(_)=>{
                return Err(ContractError::NftTransferFailed {  });
            }
            
        }

        Ok(Response::new().add_attribute("method", "manage_nft"))

    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::GetVaultId { vault_owner } => to_binary(&query::get_vault(_deps, vault_owner)?),
        QueryMsg::GetVaultArray { } =>   to_binary(&query::get_vault_array(_deps)?)
    }
}

mod query {
    

    use super::*;

    pub fn get_vault(_deps: Deps, _vault_owner: Addr) -> StdResult<GetVaultIdResponse> {
        let user_vaults = USER_VAULTS.load(_deps.storage, _vault_owner.to_string())?;
        Ok(GetVaultIdResponse { vault_id_response: user_vaults })
    }

    pub fn get_vault_array(_deps: Deps) -> StdResult<GetVaultArrayResponse> {
        let all_vaults = VAULT_LIST.load(_deps.storage)?;
        Ok(GetVaultArrayResponse { vault_array: all_vaults })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, coins};   

    #[test] 
    fn create_vault() {

        // instantiate
        let mut _deps = mock_dependencies();
        let _info = mock_info("contract_owner", &coins(1000, "denom"));
        let _msg = InstantiateMsg {};
        let _res = instantiate(_deps.as_mut(), mock_env(), _info.clone(), _msg);

        let value = STATE.load(&_deps.storage).unwrap();
        assert_eq!(_info.sender.clone(), value.contract_owner);

        // execute

        let nft_address = Addr::unchecked("nft_address");
        let name = "Reverse";
        let symbol = "REV";

        let _info_vault = mock_info("vault_owner", &coins(1000, "denom"));
        let _msg = ExecuteMsg::CreateVault { nft_asset_address: nft_address, vault_name: name.to_string(), vault_symbol: symbol.to_string() };
        let _res = execute(_deps.as_mut(), mock_env(), _info_vault.clone(), _msg);

        let id = USER_VAULTS.load(&_deps.storage, _info_vault.sender.to_string()).unwrap();

        println!("{}", id.is_empty());

        let vault = VAULT_LIST.load(&_deps.storage).unwrap();
        println!("{:?}", vault[0]);


        // query 
        let _msg = QueryMsg::GetVaultId { vault_owner: _info_vault.sender };
        let _res = query(_deps.as_ref(), mock_env(), _msg).unwrap();
        let _value: GetVaultIdResponse = from_binary(&_res).unwrap();
        println!("vault id: {}", _value.vault_id_response[0]);

        let _msg = QueryMsg::GetVaultArray { };
        let _res = query(_deps.as_ref(), mock_env(), _msg).unwrap();
        let _value: GetVaultArrayResponse = from_binary(&_res).unwrap();

        println!("vault: {:?}", _value.vault_array[0]);

        // panic!("this is value {:?}", _value)
    }
}