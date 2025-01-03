use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Protocol paused")]
    ProtocolPaused,
    
    #[msg("Address cannot be null")]
    AddressCannotBeNull,

    #[msg("Value cannot be equal")]
    ValueCannotBeEqual,
    
    #[msg("Amount over balance")]
    AmountOverBalance,
    
    #[msg("Invalid PDA")]
    InvalidPDA,

    #[msg("Bal need GT rent balance")]
    BalNeedGTRentBalance,
    
    #[msg("Other_amount_threshold cannot be zero")]
    ThresholdAmountCannotBeZero,

    #[msg("Too little output received")]
    TooLittleOutputReceived,

    #[msg("Too much input paid")]
    TooMuchInputPaid,
    
    #[msg("Arithmetic Error (overflow/underflow)")]
    ArithmeticError
}