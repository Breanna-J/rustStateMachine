//this file is for any shared types or traits that are used across the runtime. 
pub struct Block <Header, Extrinsic>{
    pub header: Header,
    pub extrinsics: Vec<Extrinsic>,
}

pub struct Header<block_number> {
    pub block_number: BlockNumber,
}

//extrinsic is a message from outside the blockchain. it tells who is makinf the call and what they want to do.
pub struct Extrinsic<Caller, Call> {
    pub Caller: Caller,
    pub Call: Call,
}

//Dispatch result will show Ok(()) if the run is successful, and if there is an error it will show the error message.
pub type DispatchResult = Result<(), &'static str>;

pub trait Dispatch{
    type caller;  //identify who called the function
    type call;  //identify what function the caller is accessing

    //get the caller and the call, and return a DispatchResult which is either Ok(()) if the call is successful, or an error message if it fails.
    fn dispatch(&mut self, caller: Self::caller, call: Self::call) -> DispatchResult;
}
