//mod declares that the module exists in the program
mod balances;   
mod system;
mod support;

//"use" imports the pallets from the balance and system crates so we can use them in our main function.
use system::Pallet as SystemPallet;
use balances::Pallet as BalancesPallet;
use crate::system::Config;


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
 