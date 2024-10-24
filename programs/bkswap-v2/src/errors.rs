use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Protocol paused")]
    ProtocolPaused,
    #[msg("Fee rate too high")]
    FeeRateTooHigh,
    #[msg("Mint is none")]
    MintIsNone,
    #[msg("User cannot be zero address")]
    UserCannotBeZeroAddress,
    #[msg("Input fee receiver is invalid")]
    InputFeeReceiverIsInvalid,
    #[msg("Amount over balance")]
    AmountOverBalance,
    #[msg("Amount cannot be zero")]
    AmountCannotBeZero,
    #[msg("Invalid PDA")]
    InvalidPDA,
    #[msg("Address cannot be null")]
    AddressCannotBeNull,
    #[msg("Value cannot be equal")]
    ValueCannotBeEqual,
    #[msg("User num too many")]
    UserNumTooMany,
    #[msg("Token num too many")]
    TokenNumTooMany,
    #[msg("Bal need GT rent balance")]
    BalNeedGTRentBalance
}