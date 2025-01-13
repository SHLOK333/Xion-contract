use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Wi-Fi configuration error")]
    WifiConfigError(),
}
