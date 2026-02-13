
//mod declares that the module exists in the program
mod balances;
mod system;

//use imports the pallets from the balance and system crates so we can use them in our main function.
use system::Pallet as SystemPallet;
use balances::Pallet as BalancesPallet;


fn main() {
    println!("Hello, world!");
}
