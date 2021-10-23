use std::{collections::HashMap, u128};

use solana_program::account_info::AccountInfo;

// TODO: sort out conversions and target caps + allocs
pub struct Presale<'a> {
    participants: Vec<AccountInfo<'a>>,
    pub whitelist: HashMap<AccountInfo<'a>, bool>,
    pub purchased: HashMap<AccountInfo<'a>, u128>,
    pub min_allocation: u128,
    pub max_allocation: u128,
    pub hardcap: u128,
    pub token_per_usd: f64,
    pub total_raised: u128,
    total_percentage_distributed: f64,
    is_active: bool,
    is_whitelist: bool,
}

impl<'a> Presale<'_> {
    pub fn new() -> Self {
        Self {
            participants: vec![],
            whitelist: HashMap::new(),
            purchased: HashMap::new(),
            // [ ] - IERC20 public tokensForSale;
            // [ ] - IERC20 public tokensBeingRaised;
            min_allocation: 100,         // const
            max_allocation: 1000,        // const
            hardcap: 400000,             // const
            token_per_usd: 3.7037037037, // const
            total_raised: 0,
            total_percentage_distributed: 0f64,
            is_active: false,
            is_whitelist: true,
        }
    }
}
