use cosmwasm_std::{
    entry_point, to_binary,   CosmosMsg, Deps, DepsMut,Binary,
    Env, MessageInfo, Response, StdResult, Uint128, WasmMsg,BankMsg,Coin
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg,JunoPunksMsg, InstantiateMsg, QueryMsg, Trait};
use crate::state::{
    CONFIG,ADMININFO,State,METADATA, AdminInfo, self
};

use cw721_base::{ExecuteMsg as Cw721BaseExecuteMsg, MintMsg};


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
         total_nft:msg.total_nft,
         owner:msg.owner,
         count:Uint128::new(0),
         check_mint:msg.check_mint,
         nft_address:"nft".to_string(),
         url : msg.url,
         price:msg.price,
         image_url:msg.image_url
    };
    CONFIG.save(deps.storage, &state)?;
    let metadata:Vec<JunoPunksMsg> = vec![];
    METADATA.save(deps.storage,&metadata)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint{rand} => execute_mint(deps, env, info,rand),
        ExecuteMsg::SetNftAddress { address } => execute_set_nft_address(deps, info, address),
        ExecuteMsg::ChangeOwner { address } => execute_chage_owner(deps, info, address),
        ExecuteMsg::SetAdmin { admin } => execute_add_metadata(deps,info,admin),
        ExecuteMsg::ChangePrice { amount }=> execute_change_price(deps,info,amount)
    }
}

fn execute_mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    rand:Uint128
) -> Result<Response, ContractError> {
    let  state = CONFIG.load(deps.storage)?;

    if state.count >= state.total_nft {
        return Err(ContractError::MintEnded {});
    }

    if rand > state.total_nft{
        return Err(ContractError::WrongNumber {  });
    }

    let amount= info
        .funds
        .iter()
        .find(|c| c.denom == "ujuno".to_string())
        .map(|c| Uint128::from(c.amount))
        .unwrap_or_else(Uint128::zero);

    if amount != state.price{
        return Err(ContractError::Notenough {});
    }

    let sender = info.sender.to_string();
    let token_id = ["Sunny".to_string(),rand.to_string()].join(".");


    
    let mut state = CONFIG.load(deps.storage)?;
    state.count = state.count+Uint128::new(1);
    state.check_mint[Uint128::u128(&rand) as usize -1] = false;
    CONFIG.save(deps.storage, &state)?;

    let admins = ADMININFO.load(deps.storage)?;
    let mut messages:Vec<CosmosMsg> = vec![];

    for admin in admins {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: admin.address,
                amount:vec![Coin{
                    denom:"ujuno".to_string(),
                    amount:admin.amount
                }]
        }));
    }
   

    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: state.nft_address,
            msg: to_binary(&Cw721BaseExecuteMsg::Mint(MintMsg {
                //::<Metadata>
                token_id: token_id.clone(),
                owner: sender,
                token_uri: Some([[state.url,rand.to_string()].join(""),"json".to_string()].join(".")),
                extension:  JunoPunksMsg{
                    name:Some("2".to_string()),
                    description:Some("desc".to_string()),
                    image:Some("image".to_string()),
                    dna:Some("dna".to_string()),
                    edition:Some(1),    
                    date:Some(123),
                    compiler:Some("compiler".to_string()),
                    attributes:vec![Trait{
                        trait_type:Some("123".to_string()),
                        value:Some("clause".to_string())
                    }]}
            }))?,
            funds: vec![],
        }))
        .add_messages(messages)
    )
    
}


fn execute_chage_owner(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
   let state =CONFIG.load(deps.storage)?;
    if state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    CONFIG.update(deps.storage,
        |mut state|-> StdResult<_>{
            state.owner = address;
            Ok(state)
        }
    )?;
    Ok(Response::default())
}


fn execute_change_price(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
   let state =CONFIG.load(deps.storage)?;
    if state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    CONFIG.update(deps.storage,
        |mut state|-> StdResult<_>{
            state.price = amount;
            Ok(state)
        }
    )?;
    Ok(Response::default())
}

fn execute_set_nft_address(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
   let state =CONFIG.load(deps.storage)?;
    if state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    CONFIG.update(deps.storage,
        |mut state|-> StdResult<_>{
            state.nft_address = address;
            Ok(state)
        }
    )?;
    Ok(Response::default())
}

pub fn execute_add_metadata(
    deps: DepsMut,
    // env:Env,
    info: MessageInfo,
    admin: Vec<AdminInfo>,
) -> Result<Response, ContractError> {
    let state =CONFIG.load(deps.storage)?;
    if state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    let mut total = Uint128::new(0);
    for one in admin.clone(){
        deps.api.addr_validate(&one.address)?;
        total = total+one.amount;
    }

    if total!= state.price{
        return Err(ContractError::WrongNumber {  })
    }

    ADMININFO.save(deps.storage,&admin)?;
    Ok(Response::default())
}


#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStateInfo {} => to_binary(& query_get_info(deps)?),
        QueryMsg::GetAdminInfo {  }=>to_binary(& query_user_info(deps)?),
        QueryMsg::GetUserInfo { address }=>to_binary(& query_info(deps,address)?)
    }
}


pub fn query_get_info(deps:Deps) -> StdResult<State>{
    let state = CONFIG.load(deps.storage)?;
    Ok(state)
}

pub fn query_user_info(deps:Deps) -> StdResult<Vec<AdminInfo>>{
   let admin = ADMININFO.load(deps.storage)?;
   Ok(admin)
}

pub fn query_info(deps:Deps,_address:String) -> StdResult<Uint128>{
    let state = CONFIG.load(deps.storage)?;
   Ok(state.total_nft)
}


pub fn query_metadata(deps:Deps) -> StdResult<Vec<JunoPunksMsg>>{
    let metadata = METADATA.load(deps.storage)?;
    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use crate::msg::Trait;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{CosmosMsg};

    #[test]
    fn buy_token() {
        let mut deps = mock_dependencies();
        let instantiate_msg = InstantiateMsg {
            total_nft:Uint128::new(10),
            owner:"creator".to_string(),
            check_mint:vec![true,true,true,true,true],
            url :"url".to_string(),
            image_url:"imag_url".to_string(),
            price:Uint128::new(10)
        };
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());
        

        let info = mock_info("creator", &[]);
        let message = ExecuteMsg::SetNftAddress { address:"nft_address1".to_string() };
        execute(deps.as_mut(), mock_env(), info, message).unwrap();

         let info = mock_info("creator", &[]);
        let message = ExecuteMsg::ChangePrice { amount:Uint128::new(12) };
        execute(deps.as_mut(), mock_env(), info, message).unwrap();

        let state =  query_get_info(deps.as_ref()).unwrap();
        assert_eq!(state.price,Uint128::new(12));
        
        
        let state= query_get_info(deps.as_ref()).unwrap();
        assert_eq!(state.nft_address,"nft_address1".to_string());

        let info = mock_info("creator", &[]);
       let message = ExecuteMsg::SetAdmin { admin: vec![AdminInfo{
            address:"admin1".to_string(),
            amount:Uint128::new(9)
       },AdminInfo{
            address:"admin2".to_string(),
            amount:Uint128::new(3)
       } ]};

       execute(deps.as_mut(), mock_env(), info, message).unwrap();
       let value =  query_info(deps.as_ref(),"1".to_string()).unwrap();

       assert_eq!(value,Uint128::new(10));

        
       let info = mock_info("creator", &[Coin{
        denom:"ujuno".to_string(),
        amount:Uint128::new(12)
       }]);
       let message = ExecuteMsg::Mint { rand: Uint128::new(1) };
       let res = execute(deps.as_mut(), mock_env(), info, message).unwrap();
       assert_eq!(res.messages.len(),3);

       assert_eq!(res.messages[1].msg,CosmosMsg::Bank(BankMsg::Send {
                to_address: "admin1".to_string(),
                amount:vec![Coin{
                    denom:"ujuno".to_string(),
                    amount:Uint128::new(9)
                }]
        }));
        
       assert_eq!(res.messages[2].msg,CosmosMsg::Bank(BankMsg::Send {
                to_address: "admin2".to_string(),
                amount:vec![Coin{
                    denom:"ujuno".to_string(),
                    amount:Uint128::new(3)
                }]
        }))
    }

}
