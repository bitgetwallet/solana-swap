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
}
