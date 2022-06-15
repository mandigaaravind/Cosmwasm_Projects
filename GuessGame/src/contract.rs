use cosmwasm_std::{
    entry_point, to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,Uint128,
};
//use random_number::random;

use crate::error::ContractError;
use crate::msg::{ArbiterResponse, ExecuteMsg, InstantiateMsg, QueryMsg, GameResult};
use crate::state::{config, config_read, State,GameRecord,resolver,resolver_read};

pub const MIN_NUMBER:u8=2;
pub const MAX_NUMBER:u8=12;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        arbiter: deps.api.addr_validate(&msg.arbiter)?,
        maxlimit:msg.maxlimit,
        minlimit:msg.minlimit,
        source: info.sender,
    };

    config(deps.storage).save(&state)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let state = config_read(deps.storage).load()?;
    match msg {
        ExecuteMsg::Play {username,amount, guessnumber} => play(deps, env, state, info, guessnumber,amount.unwrap(),username),
        ExecuteMsg::Resolve{username}=>try_approve(deps, env, state, info, username),
    }
}

fn play(
    deps: DepsMut,
    _env: Env,
    state: State,
    info: MessageInfo,
    guess_number: u8,
    amount1: Coin,
    username :String
) -> Result<Response, ContractError> {
    if MIN_NUMBER>guess_number || MAX_NUMBER<guess_number
    {
        return Err(ContractError::InvalidGuess {guess_number});
    }

    if state.maxlimit.unwrap().amount<=amount1.amount || state.minlimit.unwrap().amount>=amount1.amount
    {return Err(ContractError::InvalidBetAmount {})};
     
    let key = username.as_bytes();

    if (resolver_read(deps.storage).may_load(key)?).is_some() {
        return Err(ContractError::RecordAlreadyExist{userame:username.clone()});
    }
   

    let dice1 =6;
    let dice2 =9;
    
    let sum_die:u8 = dice1+dice2;
    
    let mut winner =false;
    if sum_die==guess_number{
        winner=true;
    }
  
    let record = GameRecord { owner: info.sender,
        sum_prediction : guess_number,
        sum_actual : sum_die,
        sum_dice1 :dice1,
        sum_dice2 : dice2,
        is_winner : winner,
        entry_fee : Some(amount1.clone()),
     };
    
    resolver(deps.storage).save(key, &record)?;
  
    Ok(Response::default())
  }

fn try_approve(
    deps: DepsMut,
    _env: Env,
    state: State,
    info: MessageInfo,
    name : String,
) -> Result<Response, ContractError> {
    if info.sender != state.arbiter {
        return Err(ContractError::Unauthorized {});
    }

    let key = name.as_bytes();
    
    if !(resolver_read(deps.storage).may_load(key)?).is_some() {
        return Err(ContractError::RecordAlreadyExist{userame:name.clone()});
    }
    let _state = config_read(deps.storage).load()?;
    let address = match resolver_read(deps.storage).may_load(key)? {
        Some(record) => Some(record.owner),
        None => None,
    };

    let result =match resolver_read(deps.storage).may_load(key)? {
        Some(record) => Some(record.is_winner),
        None => None,
    };
    
    let boolean_result =result.unwrap();
    let amount3=match resolver_read(deps.storage).may_load(key)? {
        Some(record) => Some(record.entry_fee),
        None => None,
    };
    let x2=amount3.clone();
    let x=Uint128::from(2u8);
    let total_money=Coin{
        amount : amount3.unwrap().unwrap().amount*x,
        denom : x2.unwrap().unwrap().denom
    };
     let add =address.unwrap();
    if boolean_result {

        Ok(send_tokens(add, vec!(total_money), "approve"))
    }

    else { Err(ContractError::CannotSendReward{})}
}

fn send_tokens(to_address: Addr, amount: Vec<Coin>, action: &str) -> Response {
    Response::new()
        .add_message(BankMsg::Send {
            to_address: to_address.clone().into(),
            amount:amount,
        })
        .add_attribute("action", action)
        .add_attribute("to", to_address)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Arbiter {} => to_binary(&query_arbiter(deps)?),
        QueryMsg::BetResult{username} =>to_binary(&query_result(deps,username)?),
    }
}


fn query_result(deps: Deps,username :String) -> StdResult<GameResult> {
    let key = username.as_bytes();
    let result =match resolver_read(deps.storage).may_load(key)? {
        Some(record) => Some(record.is_winner),
        None => None,
    };
    let boolean_result_rslt =result.unwrap();
    if boolean_result_rslt{
        Ok(GameResult { gameresult: "Won".to_string() })
    }

    else  {
        Ok(GameResult { gameresult: "Lost".to_string() })
    }

    
}

fn query_arbiter(deps: Deps) -> StdResult<ArbiterResponse> {
    let state = config_read(deps.storage).load()?;
    let addr = state.arbiter;
    Ok(ArbiterResponse { arbiter: addr })
}

