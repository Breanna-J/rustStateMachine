//mod declares that the module exists in the program
mod balances;
mod system;

//use imports the pallets from the balance and system crates so we can use them in our main function.
use system::Pallet as SystemPallet;
use balances::Pallet as BalancesPallet;

//asks rust to allow you to print the type  in human readible text for debugging; all of the pallets associated need the derive(debug) statement
#[derive(Debug)]

//this is the main runtime, it accumulates all the different pallets in one
//struct is the definition of the data
pub struct Runtime {
    //SystemPallet and BalancePallet are imported above in the use statement
    system : SystemPallet,
    balances : BalancesPallet,
}
//impl is what it can do
impl Runtime {
    //create an instance of the main runtime by creating new instatnces of the pallets
    fn new()-> Self{
        Self { 
            system: SystemPallet::new(), 
            balances: BalancesPallet::new() 
        }
    }
}

fn main() {
    println!("Hello, world!");
}
