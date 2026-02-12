//This is a ballances module.
//This module keeps track of track of the balance of each account in the system.

//Need to import the BTreeMap from the standard library to use it in our Pallet struct.
use std::collections::BTreeMap;
#[derive(Debug, Clone)]
// a struct is an object that can hold multiple values of different types.
pub struct Pallet{
    // storage mapping the account ('String') to the balance (u128).
    account: String,
    balance: u128,
    balances: BTreeMap<String, u128>

}



impl Pallet {
    pub fn new() -> Self {
        Self {
            account: String::new(),
            balance: 0,
            balances: BTreeMap::new() 
            }
    }
}