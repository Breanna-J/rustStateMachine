//This is a ballances module.
//This module keeps track of track of the balance of each account in the system.
//Need to import the BTreeMap from the standard library to use it in our Pallet struct.


use std::{collections::BTreeMap, iter::empty};
use num_traits::{Zero, CheckedAdd, CheckedSub};

use crate::types::{Account, Balance};

pub trait Config: crate::system::Config {
    //adding crate::system::Config as a supertrait means 
    //that any type that implements Config must also 
    //implement crate::system::Config. This allows us to 
    //use the types defined in crate::system::Config 
    //(like Account and BlockNumber) in our balances module,
    // while still keeping the balances module separate 
    //and modular.  
    type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}

#[derive(Debug)]
pub struct Pallet<T:Config>{
   balances : BTreeMap<T::Account, T::Balance>,
}

impl <T:Config> Pallet<T>
where 
    Account: Ord + Clone, //orderable and dublicatable
    Balance: Zero + CheckedSub + CheckedAdd + Copy, //can be zero, can support safe arrethmatic, cheap to duplicate without moving
{
    pub fn new() -> Self {
        Self {
            balances : BTreeMap::new() 
            }
    }

    //set the balance of an account.
    pub fn set_balance(&mut self, account:&T::Account, balance: T::Balance) {
        self.balances.insert(account.clone(), balance);
    }

    //get the balance of an account.
    //if the account does not exist, return 0.
    //the return type is Option<&u128> because we want to return a reference
    //to the balance if it exists, or None if it does not exist.
    pub fn get_balance(&self, account: &T::Account) -> T::Balance{
        *self.balances.get(account).unwrap_or(&T::Balance::zero())
    }
    //function to transfer balance from one account to another.
    pub fn transfer(
        &mut self,
        caller: T::Account,
        to: T::Account,
        amount:T::Balance,
    ) -> Result<(), &'static str> {

            //check the sender has enough balance to transfer.
            let caller_balance = self.get_balance(&caller);
            let to_balance = self.get_balance(&to);
            
            //check if the caller has enough balance to transfer the amount, and if the recipient's balance will not overflow after the transfer
            // perform the math safely using checked_sub and checked/add to prevent overflow and underflow, and return an error if the math is not correct.
            let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Insufficient balance")?;
            
            //? means if the result is an error, return the error immediately, otherwise continue with the value.
            let new_to_balance = to_balance.checked_add(&amount).ok_or("Balance overflow")?;

            //if the caller has enough balance, transfer the amount from the caller to the recipient and update both balances.
            self.set_balance(&caller, new_caller_balance);
            self.set_balance(&to, new_to_balance);

            //return Ok if the math is correct and the transfer is successful.
            Ok(())
        }
    }   
 
//this is a conditional compilation attribute that tells the compiler to only compile when running tests. This is useful for keeping test code separate from production code.
#[cfg(test)]
mod tests {   
    struct TestConfig;
    impl crate::system::Config for TestConfig{
        type Account = String;
        type BlockNumber = u32;

        type Nonce = u32;
    } 

    impl super::Config for TestConfig {
        type Balance = u128;
    }
    
    //singular test
    #[test]
    //CREATE A NEW PALLET then set it balance for Alice to 0 and then get the balance for Alice and assert that it is 0.   
    fn balance_tests(){         
        //super::pallet::new() is used to grab the function from the parent module, without importing it.
        //if you want to import the whole thing (for example if there are sever structs or functions with the same name)
        //you would use super::*; to import everything from the parent module.
        //self=> the current module, super => the parent module(current file), crate=> the root module(parent directory.
        use crate::balances::Pallet;
        let mut balances: Pallet<TestConfig> = super::Pallet::<TestConfig>::new();
       
        balances.set_balance(&"Alice".to_string(), 0);
        balances.set_balance(&"Bob".to_string(), 0);

        let res = balances.transfer("Alice".to_string(), "Bob".to_string(), 40);
        assert_eq!(res, Ok(()));

        assert_eq!(balances.get_balance(&&"Alice".to_string()), 100);
        assert_eq!(balances.get_balance(&&"Bob".to_string()), 60);
    }


}   