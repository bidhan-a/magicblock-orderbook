use anchor_lang::error_code;

#[error_code]
pub enum Error {
    #[msg("The mint is invalid for the provided orderbook.")]
    InvalidMint,
    #[msg("The user is not authorized.")]
    Unauthorized,
    #[msg("Insufficient balance")]
    InsufficientBalance,
}
