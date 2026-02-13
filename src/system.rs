use std::collections::BTreeMap;
use crate::balances::Pallet as BalancesPallet;

pub struct Pallet {
    //current block number
    block_number: u32,
    // storage mapping the account ('String') to the balance (u128).
    nonce: BTreeMap<String, u32>
}

impl Pallet {
    //create a new instance of the pallet with an initial block number of 0 and an empty nonce mapping.
    pub fn new() -> Self {
        return Self {
            block_number: 0,
            nonce: BTreeMap::new()
        }
    }
}
