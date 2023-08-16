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

    #[error("distribute error")]
    DistributeError {},

    #[error("farming incentive error")]
    FarmingIncentiveFundError {},

    #[error("dev address error")]
    DevAddrError {},

    #[error("burn error")]
    BurnError {},

    #[error("Error:{e1}")]
    Testing{e1: String}
}
