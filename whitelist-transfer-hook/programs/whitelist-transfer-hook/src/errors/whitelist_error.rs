use anchor_lang::error_code;

#[error_code]
pub enum WhitelistError {
    #[msg("This account is already whitelisted")]
    AlreadyWhitelisted,
    #[msg("This account is not whitelisted")]
    AccountNotWhitelisted,
    #[msg("This account does not exist")]
    WhitelistAccountNotExist,
}