use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Protocol paused")]
    ProtocolPaused,
    #[msg("Amount over balance")]
    AmountOverBalance,
    #[msg("Amount is 0")]
    AmountIsZero,
    #[msg("Invalid input param")]
    InvalidInputParam,
    #[msg("The input address cannot be zero")]
    TheInputAddressCannotBeZero,
    #[msg("Transfer amount need GT 0")]
    TransferAmountNeedGT0,
    #[msg("Too little output received")]
    TooLittleOutputReceived,
    #[msg("Too much input paid")]
    TooMuchInputPaid,
    #[msg("Arithmetic Error (overflow/underflow)")]
    ArithmeticError,

}