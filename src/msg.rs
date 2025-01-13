use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::state::WifiConfigResponse;


#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    ConfigureWifi { ssid: String, password: String },
    UpdateSignalStrength { strength: u8 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(WifiConfigResponse)]
    GetWifiConfig {},
}
