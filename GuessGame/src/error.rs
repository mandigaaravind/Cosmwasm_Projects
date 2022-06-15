use cosmwasm_std::{StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Guessed Number not in range (Guess_Number {guess_number:?}")]
    InvalidGuess {guess_number:u8},

    #[error("Unauthorized")]
    Unauthorized {},
     
    #[error("Invalid Bet amount provided .")]
    InvalidBetAmount {},
     
    #[error("Record already exist (Username {userame:?}")]
    RecordAlreadyExist{userame:String},
    
    #[error("Record not present for (Username {userame:?}")]
    RecordNotPresentToApprove{userame:String},
    

    #[error("Cannot send reward , the user has lost the bet")]
    CannotSendReward{},
}
