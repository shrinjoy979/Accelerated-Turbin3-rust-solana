use anchor_lang::{
    prelude::*, 
    system_program
};
use anchor_spl::token_interface::spl_pod::option::Nullable;

use crate::{errors::WhitelistError, state::whitelist::Whitelist};

#[derive(Accounts)]
#[instruction(user_ata: Pubkey)]
pub struct WhitelistOperations<'info> {
    #[account(
        mut,
        //address = 
    )]
    pub admin: Signer<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"whitelist", user_ata.key().as_ref()],
        space = 8 + Whitelist::DISCRIMINATOR.len() + Whitelist::INIT_SPACE,
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistOperations<'info> {
    pub fn add_to_whitelist(&mut self, address: Pubkey, bumps: &WhitelistOperationsBumps) -> Result<()> {

        // check that the account exists and is_whitelisted
        let whitelist = &mut self.whitelist;

        if whitelist.is_whitelisted {
            return err!(WhitelistError::AlreadyWhitelisted);
        }

        whitelist.set_inner(Whitelist { user_address: address, is_whitelisted: true, bump: bumps.whitelist });
        Ok(())
    }

    pub fn remove_from_whitelist(&mut self) -> Result<()> {
        let whitelist = &mut self.whitelist;

        if !whitelist.is_whitelisted {
            return err!(WhitelistError::AccountNotWhitelisted);
        }

        // if !Pubkey::is_some(&whitelist.user_address.key()) {
        //     return err!(WhitelistError::WhitelistAccountNotExist);
        // }

        whitelist.is_whitelisted = false;
        Ok(())
    }
}
