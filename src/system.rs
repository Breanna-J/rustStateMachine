use std::{collections::BTreeMap, ops::AddAssign};
use num::traits::{One, Zero};

use crate::types::Account;

 
//allows to debug in the main runtime
#[derive(Debug)]
pub struct Pallet <BlockNumber, Account, Nonce> {
    //current block number
    block_number: BlockNumber,
    // storage mapping the account ('String') to the balance (u128).
    //nonce is a number used once. This is used to prevent replay attacks and ensure that each transaction is unique. In this case,
    // we are using a BTreeMap to store the nonce for each account, where the key is the account (String) and the value is the nonce (u32).
    nonce: BTreeMap<Account, Nonce> 
}

impl <BlockNumber, Account, Nonce> Pallet <BlockNumber, Account, Nonce> 
where
    BlockNumber: Zero + One + AddAssign + Copy,
    Account: Ord + Clone,
    Nonce: Zero + One + Copy,
{
    //create a new instance of the pallet with an initial block number of 0 and an empty nonce mapping.
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new()
        }
    }
    //Get the current block number. This function copies the block_number and returns it, to maintain ownership.
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    //this simulates the passage of time in the blockchain, as each block is produced.
    pub fn increment_block(&mut self) {
        //increment the block number by one
        self.block_number += BlockNumber::one();
    }

    pub fn increment_nonce(&mut self, account: &Account) {
        let nonce: Nonce = *self.nonce.get(account).unwrap_or(&Nonce::zero());
        let new_nonce: Nonce = nonce + Nonce::one();
        self.nonce.insert(account.clone(), new_nonce);
    }

}

#[cfg(test)]
mod test{
    use crate::types::{BlockNumber, Account, Nonce};


    fn init_system(){
        //import Pallet
        use super::*;
       
        //Create a new instance
        let mut system: Pallet<BlockNumber, Account, Nonce> = super::Pallet::new();

        //increment the block
        system.increment_block();

        //increment the nonce for the Alice account
        system.increment_nonce(&"Alice".to_string());

        //assert that the first part = the second number
        assert_eq!(system.block_number(), 1);
        assert_eq!(system.nonce.get("Alice"), Some(&Nonce::one()));
        assert_eq!(system.nonce.get("Bob"), None);
    }
}
