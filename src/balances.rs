//This is a ballances module.
//This module keeps track of track of the balance of each account in the system.

//Need to import the BTreeMap from the standard library to use it in our Pallet struct.

use std::collections::BTreeMap;

#[derive(Debug, Clone)]

// a struct is an object that can hold multiple values of different types, like a class.

pub struct Pallet{
    // storage mapping the account ('String') to the balance (u128).
    balances: BTreeMap<String, u128>

}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new() 
            }
    }

    //set the balance of an account.

    pub fn set_balance(&mut self, account: String, balance: u128) {
        self.balances.insert(account, balance);
    }

    //get the balance of an account.
    //if the account does not exist, return 0.
    //the return type is Option<&u128> because we want to return a reference
    //to the balance if it exists, or None if it does not exist.

    pub fn get_balance(&self, account: &String) -> u128 {
        *self.balances.get(account).unwrap_or(&0)
    }
}   
//this is a conditional compilation attribute that tells the compiler to only compile when running tests. This is useful for keeping test code separate from production code.
#[cfg(test)]
mod tests {
    //singular test
    #[test]

    //CREATE A NEW PALLET then set it balance for Alice to 0 and then get the balance for Alice and assert that it is 0.
    
    
    fn balance_tests(){ 


        //super::pallet::new() is used to grab the function from the parent module, without importing it.
        //if you want to import the whole thing (for example if there are sever structs or functions with the same name)
        //you would use super::*; to import everything from the parent module.
        
        //self=> the current module, super=> the parent module(current file), crate=> the root module(parent directory.
        
        let mut pallet = super::Pallet::new();

        assert_eq!(pallet.set_balance("Alice".to_string(), 0), ());
        assert_eq!(pallet.set_balance("Bob".to_string(), 0), ());
        pallet.set_balance("Alice".to_string(), 100);
        assert_eq!(pallet.get_balance(&"Alice".to_string()), 100);
        assert_eq!(pallet.get_balance(&"Bob".to_string()), 0);
    }


}