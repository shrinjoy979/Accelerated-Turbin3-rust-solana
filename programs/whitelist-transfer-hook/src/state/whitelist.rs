use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Whitelist {
    /* this causes bottlenecks i.e - 1 keep adding pubkeys to this vec 300,000. The address, we want to blacklist is at 299,999
        the program will be slow and more compute units
    */
    pub address: Pubkey,
    pub is_whitelisted: bool,
    pub bump: u8,
}
/*  create PDA per user - 
    

*/