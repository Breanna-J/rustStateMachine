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
    //function to transfer balance from one account to another.
    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), String> {

            //check the sender has enough balance to transfer.
            let caller_balance = self.get_balance(&caller);
            let to_balance = self.get_balance(&to);
            
            //check if the caller has enough balance to transfer the amount, and if the recipient's balance will not overflow after the transfer
            //perform the math safely using checked_sub and checked_add to prevent overflow and underflow, and return an error if the math is not correct.
            let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Insufficient balance".to_string())?;
            
            //? means if the result is an error, return the error immediately, otherwise continue with the value.
            let new_to_balance = to_balance.checked_add(amount).ok_or("Balance overflow".to_string())?;

            //if the caller has enough balance, transfer the amount from the caller to the recipient and update both balances.
            self.set_balance(caller, new_caller_balance);
            self.set_balance(to, new_to_balance);

            //return Ok if the math is correct and the transfer is successful.
            Ok(())
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