#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{WifiConfigResponse, RESULT};

const CONTRACT_NAME: &str = "crates.io:wifi-config";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let wifi_config = WifiConfigResponse {
        ssid: String::from("default_network"),
        password: String::from("password123"),
        signal_strength: 0,
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();
    RESULT.save(deps.storage, &wifi_config).unwrap();
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ConfigureWifi { ssid, password } => execute::configure_wifi(deps, ssid, password),
        ExecuteMsg::UpdateSignalStrength { strength } => execute::update_signal_strength(deps, strength),
    }
}

pub mod execute {
    use super::*;

    pub fn configure_wifi(deps: DepsMut, ssid: String, password: String) -> Result<Response, ContractError> {
        let wifi_config = WifiConfigResponse { ssid, password, signal_strength: 0 };
        RESULT.save(deps.storage, &wifi_config).unwrap();

        let res = Response::new().add_attributes(vec![
            ("action", "configure_wifi"),
            ("ssid", &wifi_config.ssid),
            ("password", &wifi_config.password),
        ]);
        Ok(res)
    }

    pub fn update_signal_strength(deps: DepsMut, strength: u8) -> Result<Response, ContractError> {
        let mut wifi_config = RESULT.load(deps.storage).unwrap();
        wifi_config.signal_strength = strength;
        RESULT.save(deps.storage, &wifi_config).unwrap();

        let res = Response::new().add_attributes(vec![
            ("action", "update_signal_strength"),
            ("strength", &strength.to_string()),
        ]);
        Ok(res)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::GetWifiConfig {} => query::get_wifi_config(deps),
    }
}

pub mod query {
    use super::*;

    pub fn get_wifi_config(deps: Deps) -> Result<QueryResponse, StdError> {
        let result = RESULT.load(deps.storage)?;
        to_json_binary(&result)

    }
}
