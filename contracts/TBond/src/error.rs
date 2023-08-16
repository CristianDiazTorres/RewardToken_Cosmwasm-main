use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    
    #[error("unauthorized error")]
    Unauthorized {},

    #[error("zero address")]
    ZeroAddrError {},

    #[error("burn error")]
    BurnError {},

    #[error("allowance error")]
    AllowanceError {},

    #[error("Error:{e1}")]
    Testing{e1: String}
}
