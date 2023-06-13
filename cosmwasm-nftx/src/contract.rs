#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr};
use cw2::set_contract_version; 

use crate::state::{ State, STATE, Vault, USER_VAULTS, VAULT_LIST };
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, GetVaultResponse, GetVaultArrayResponse};

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
    }
}

pub mod execute {
    use super::*;
    
    pub fn create_vault(_deps: DepsMut, _env: Env, _info: MessageInfo, _nft_asset_address: Addr, _vault_name: String, _vault_symbol: String) -> Result<Response, ContractError> {

        // let vault_id_size = VAULT_LIST.load(_deps.storage).unwrap();
        

        // let vault = Vault {
        //     vault_id: 1,
        //     vault_name: _vault_name,
        //     vault_symbol: _vault_symbol,
        //     nft_asset_address: _nft_asset_address
        // };

        // let user_vault = USER_VAULTS.load(_deps.storage, _info.sender.to_string())?;

        // if user_vault.len() > 0 {
        //     USER_VAULTS.update(_deps.storage, _info.sender.to_string(), |vault_array| -> Result<Vec<Vault>, ContractError> {
        //         match vault_array {
        //             Some(mut vec_arr) => {
        //                 vec_arr.push(vault.clone());
        //                 Ok(vec_arr)
        //             },
        //             None => {
        //                 return Err(ContractError::Unauthorized {  })
        //             }
        //         }
        //     })?;
        // } else {
            // let mut vault_array: Vec<Vault> = Vec::new(); vault_array.push(vault.clone());
        // } 
        
        

        // let update_vault_array = |mut vault_array: Vec<Vault>| -> Result<Vec<Vault>, ContractError> {
        //     vault_array.push(vault.clone());
        //     Ok(vault_array)
        // };

        // VAULT_LIST.update(_deps.storage, update_vault_array)?;

        Ok(Response::new().add_attribute("method", "create_vault"))
    }
}

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
//     match _msg {
//         QueryMsg::GetVault { vault_owner } => to_binary(&query::get_vault(_deps, vault_owner)?),
//         QueryMsg::GetVaultArray { } =>   to_binary(&query::get_vault_array(_deps)?)
//     }
// }

// mod query {
    

//     use super::*;

//     pub fn get_vault(_deps: Deps, _vault_owner: Addr) -> StdResult<GetVaultResponse> {
//         let user_vaults = USER_VAULTS.load(_deps.storage, _vault_owner.to_string());
//         Ok(GetVaultResponse { vault_response: user_vaults.unwrap() })
//     }

//     pub fn get_vault_array(_deps: Deps) -> StdResult<GetVaultArrayResponse> {
//         unimplemented!()
//     }
// }

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

        // let nft_address = Addr::unchecked("nft_address");
        // let name = "Reverse";
        // let symbol = "REV";

        // let _info_vault = mock_info("vault_owner", &coins(1000, "denom"));
        // let _msg = ExecuteMsg::CreateVault { nft_asset_address: nft_address, vault_name: name.to_string(), vault_symbol: symbol.to_string() };
        // let _res = execute(_deps.as_mut(), mock_env(), _info_vault, _msg);

        // assert_eq!(_info_vault.clone().sender, _info_vault.sender);

        // let test = USER_VAULTS.load(&_deps.storage, _info_vault.clone().sender.to_string());

        // for i in test.unwrap().iter() {
        //     println!("{:?}", i);
        // }

        // let test: bool = USER_VAULTS.is_empty(&_deps.storage);
        // println!("{}", test);

        // let test = VAULT_LIST.may_load(&_deps.storage);
        // println!("{:?}", test);


        // assert_eq!(1, test.len());

        // query 
        // let _msg = QueryMsg::GetVault { vault_owner: _info_vault.sender };
        // let _res = query(_deps.as_ref(), mock_env(), _msg);

        

        // match _res {
        //     Ok(res) => {
        //         let value: GetVaultResponse = from_binary(&res).unwrap();
        //         panic!("this is the value {:?}", value);
        //     },
        //     Err(err) => panic!("This is the error {}", err)
        // }
           
        // let _value: GetVaultResponse = from_binary(&_res).unwrap();

        // panic!("this is value {:?}", _value)
    }
}