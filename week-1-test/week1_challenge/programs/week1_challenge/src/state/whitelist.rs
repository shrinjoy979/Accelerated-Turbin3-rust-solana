use anchor_lang::prelude::*;
// pub use shared_types::Whitelist;

#[account]
// #[derive(InitSpace)] can't use initspace since i'm using a dynamic vec
pub struct Whitelist {
    pub address: Vec<(Pubkey, u64, bool)>,
    pub whitelist_bump: u8,
    pub admin: Pubkey,
}

impl Whitelist {
    pub fn contains_address(&self, address: &Pubkey) -> bool {
        self.address.iter().any(|(addr, _, _)| addr == address)
    }

    pub fn is_whitelisted(&self, address: &Pubkey) -> Option<&bool> {
        let user_is_whitelisted = self
            .address
            .iter()
            .find(|(addr, _, _)| *address == *addr)
            .map(|(_, _, is_whitelisted)| is_whitelisted);
        user_is_whitelisted
    }
}

// pub fn test() {
//     let new_whitelist: Whitelist = Whitelist {
//         address: vec![(
//             Pubkey::from_str_const("AkTTSsoAmjbmDQTVzFmoEWJ5o2j78xGBtSVuB68irJiJ"),
//             20,
//         )],
//         whitelist_bump: 2,
//     };

//     let is_whitelist = new_whitelist.contains_address(&Pubkey::from_str_const(
//         "AkTTSsoAmjbmDQTVzFmoEWJ5o2j78xGBtSVuB68irJiJ",
//     ));
// }
