//mod declares that the module exists in the program
mod balances;   
mod system;
mod support;

use std::usize;

//"use" imports the pallets from the balance and system crates so we can use them in our main function.
use system::Pallet as SystemPallet;
use balances::Pallet as BalancesPallet;
use support::Extrinsic;
use crate::{support::Extrinsic, system::Config, types::{BlockNumber, Nonce}};


//We can make the types explicit, this makes it easier to reffactor and change the overall type/constrain the code
pub mod types{
    pub type Account = String;
    pub type Balance = u128;
    pub type Nonce = u32;
    pub type BlockNumber = u32;
    //all envocable functions in the runtime need to be defined in one place. 
    pub type Extrinsic = crate::support::Extrinsic<Account, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
}
//asks rust to allow you to print the type  in human readible text for debugging; all of the pallets associated need the derive(debug) statement
#[derive(Debug)]
pub enum RuntimeCall {
    
}
//this is the main runtime, it accumulates all the different pallets in one
//struct is the definition of the data
pub struct Runtime {
    //SystemPallet and BalancePallet are imported above in the use statement
    //system : system::Pallet<types::BlockNumber, types::Account, types::Nonce>,
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}
//impl is what struct can do
impl system::Config for Runtime{
    //pointing the generic type to another generic type makes them all consistently one type
    type Account = types::Account;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}
impl Runtime {

    //run a block of extrinsics and increment the block number.
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        //inccrement the block and check if the block number is correct, if not return an error message.
        self.system.increment_block();
            if block.header.block_number != self.system.block_number(){
                return Err("Block Number {block.header.block_number} invalid. Expected {self.system.block_number()}");
            }
        //execute each extrinsic in the block and check if the extrinsic is valid, if not return an error message.
        //Can use iter or into_iter, iter borrows the data and leaves it in place, into_iter takes ownership of the data 
        //and moves it, and iter_mut allows you to mutate the data while iterating over it.
        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.increment_nonce(&caller);
            //dispatch the extrinsic and check if it is valid, if not return an error message with the index of the extrinsic in the block.
            let _res: Result<(), ()> = self.dispadtch(caller, call).map_err(|e| format!("Error executing extrinsic {i}: {e}"))?;
        }
        Ok(())
    }
    //create an instance of the main runtime by creating new instatnces of the pallets
    fn new()-> Self{
        Self { 
            system: system::Pallet::new(), 
            balances: balances::Pallet::new() 
        }
    }
}

fn main() {
    //start the runtime
    let mut runtime: Runtime = Runtime::new();

    //initalize the accounts
    let alice: String = "Alice".to_string();
    let bob: String = "Bob".to_string();
    let charlie: String = "Charlie".to_string();

    //set balance for Alice
    runtime.balances.set_balance(&alice, 100);

    //start emulating the block
    runtime.system.increment_block();
    assert_eq!(runtime.system.block_number(), 1);

    //first transaction
    runtime.system.increment_nonce(&alice);
    let _res: Result<(), ()> = runtime.balances.transfer(alice.clone(), bob,30).map_err( |e| eprintln!("{e}"));

    //second transaction
    runtime.system.increment_nonce(&alice);
    let _res: Result<(), ()> = runtime.balances.transfer(alice.clone(), charlie,15).map_err( |e| eprintln!("{e}"));

}
 