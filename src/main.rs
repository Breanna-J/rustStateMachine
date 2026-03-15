//mod declares that the module exists in the program
mod balances;   
mod system;
mod support;

use std::usize;

//"use" imports the pallets from the balance and system crates so we can use them in our main function.
use system::Pallet as SystemPallet;
use balances::Pallet as BalancesPallet;
use crate::{support::Extrinsic, system::Config, types::{Block, BlockNumber, Nonce}, support::Dispatch};


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
    //the transfer call takes in a destination account and an amount to transfer.
    Transfer { to: types::Account, amount: types::Balance },

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
        if block.header.block_number != self.system.block_number() {
            return Err("");
        }
        //execute each extrinsic in the block and check if the extrinsic is valid, if not return an error message.
        //Can use iter or into_iter, iter borrows the data and leaves it in place, into_iter takes ownership of the data 
        //and moves it, and iter_mut allows you to mutate the data while iterating over it.
        for (_i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.increment_nonce(&caller);
            //dispatch the extrinsic and check if it is valid, if not return an error message with the index of the extrinsic in the block.
            self.dispatch(caller, call)?;
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


//dispatch the call on behlaf of the caller
impl crate::support::Dispatch for Runtime  {
    type Caller = <Runtime as system::Config>::Account;
    type Call = RuntimeCall;
    //
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> support::DispatchResult {
        match call {
            RuntimeCall::Transfer { to, amount } => {
                //the ? tells the program that this could be an error, and if it is an error, 
                //return the error immediately, otherwise continue with the value.  
                self.balances.transfer(caller, to, amount)?;
                Ok(())
            }
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

    //set initial balance for Alice
    runtime.balances.set_balance(&alice, 100);

    //Set extrinsics for the block. Extrinsics are data that comes from outside the blockchain, and they tell the blockchain what to do.
    //In this case, we are creating a block with two extrinsics. The extrinsics are wrapped in a block, which also contains a header with the block number. 
    //This simulates a real block being produced on the blockchain, with transactions being included in the block and executed by the runtime.
    //These can be added or removed as needed based on modules and calls used
    let block_1: support::Block<support::Header<u32>, support::Extrinsic<String, RuntimeCall>> = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Transfer { to: bob.clone(), amount: 30 },
            },
            Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Transfer { to: charlie.clone(), amount: 15 },
            },
        ],
    };

    //execute the extrinsics that make up the block and check if the block is valid, if not print the error message.
    runtime.execute_block(block_1).unwrap_or_else(|e| eprintln!("{e}"));
    
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
 