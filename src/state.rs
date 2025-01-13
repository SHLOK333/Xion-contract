use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

#[cw_serde]
pub struct WifiConfigResponse {
    pub ssid: String,
    pub password: String,
    pub signal_strength: u8,
}

// Mapping the Wi-Fi configuration result

pub const RESULT: Item<WifiConfigResponse> = Item::new("wifi_config");
