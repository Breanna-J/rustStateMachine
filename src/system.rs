use std::collections::BTreeMap;

pub struct Pallet {
    //current block number
    block_number: u32,
    // storage mapping the account ('String') to the balance (u128).
    //nonce is a number used once. This is used to prevent replay attacks and ensure that each transaction is unique. In this case,
    // we are using a BTreeMap to store the nonce for each account, where the key is the account (String) and the value is the nonce (u32).
    nonce: BTreeMap<String, u32> 
}

impl Pallet {
    //create a new instance of the pallet with an initial block number of 0 and an empty nonce mapping.
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new()
        }
    }
    //get the current block number.
    pub fn block_number(&self) -> u32 {
        self.block_number
    }
    //increment the block number by 1.
    //this simulates the passage of time in the blockchain, as each block is produced.
    pub fn increment_block(&mut self) {
        self.block_number += 1;
    }

    pub fn increment_nonce(&mut self, account: &String) {
        let current_nonce = self.nonce.get(&account).unwrap_or(0);
        let new_nonce: u32 = current_nonce + 1;
        self.nonce.insert(account.clone(), new_nonce + 1);
    }

}

#[cfg(test)]
mod test{
    fn init_system(){
        let mut system: Pallet = super::Pallet::new();
        system.increment_block();
        system.increment_nonce(&"Alice".to_string());

        assert!(system.block_number(), 1);
        assert!(system.nonce.get("Alice"), Some(&1));
        assert!(system.nonce.get("Bob"), None);
    }
}
