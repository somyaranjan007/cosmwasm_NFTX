#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr};
use cw2::set_contract_version; 



use cw20_base::ContractError;
use cw20_base::contract::{execute_mint , execute_burn , execute_transfer};

use crate::msg::{VtokenInstantiateMsg,VtokenExecuteMsg,VtokenQueryResponses};
use crate::state::{VTOKEN_STATE,VTokenState};


const CONTRACT_NAME: &str = "crates.io:cosmwasm-vtokens";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature="library"), entry_point)]
pub fn vtoken_instantiate(_deps:DepsMut,_env:Env,_info:MessageInfo,_msg:VtokenInstantiateMsg,) ->Result<Response,ContractError>{
    set_contract_version(_deps.storage,CONTRACT_NAME , CONTRACT_VERSION)?;

    let vtoken_state =VTokenState{
         vtoken_name : _msg.name,
         vtoken_symbol: _msg.symbol ,
         vtoken_nft_id: _msg.nft_id,
         user_address:_info.sender,
        };


        VTOKEN_STATE.save(_deps.storage,&vtoken_state);

        Ok(Response::new().add_attribute("method", "instantiate"))
        // unimplemented!()

}

#[cfg_attr(not(feature="library"), entry_point)]
pub fn vtoken_execute(_deps:DepsMut,_env:Env,_info:MessageInfo,_msg:VtokenExecuteMsg)->Result<Response,ContractError>{
    match _msg{
        VtokenExecuteMsg::Mint { account, amount }=> execute_mint(_deps, _env, _info, account.to_string(), amount),
        VtokenExecuteMsg::Burn {  amount }=>execute_burn(_deps, _env, _info, amount),
        VtokenExecuteMsg::Transfer { to_account, amount }=>execute_transfer(_deps, _env, _info, to_account.to_string(), amount)
    }

}











