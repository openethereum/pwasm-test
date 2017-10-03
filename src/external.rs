use parity_hash::{ H256, Address };
use bigint::{ U256 };

pub struct Error;

pub trait External {
    fn storage_read(&mut self, key: &H256, value: &mut [u8]) -> i32 {
        unimplemented!();
    }
    fn storage_write(&mut self, key: &H256, value: &[u8]) -> i32 {
        unimplemented!();
    }
    fn suicide(&mut self, refund: &Address) {
        unimplemented!();
    }
    fn create(&mut self, endowment: U256, code: &[u8]) -> Result<Address, Error> {
        unimplemented!();
    }
    fn call(&mut self, address: &Address, val: U256, input: &[u8], result: &mut [u8]) -> i32 {
        unimplemented!();
    }
    fn call_code(&mut self, address: &Address, input: &[u8], result: &mut [u8]) -> i32 {
        unimplemented!();
    }
    fn static_call(&mut self, address: &Address, input: &[u8], result: &mut [u8]) {
        unimplemented!();
    }
    fn debug_log(&mut self, _msg: String) {

    }
    fn gas_limit(&mut self, dst: &[u8]) {
        unimplemented!();
    }
    fn blockhash(&mut self, number: i64, dest: &[u8]) -> i32 {
        unimplemented!();
    }
    fn coinbase(&mut self) {
        unimplemented!();
    }
    fn timestamp(&mut self) -> i64 {
        unimplemented!();
    }
    fn blocknumber(&mut self) -> i64 {
        unimplemented!();
    }
    fn difficulty(&mut self, dest: &[u8]) {
        unimplemented!();
    }
    fn value(&mut self) -> U256 {
        unimplemented!();
    }
}

pub struct ExternalImpl {}

impl ExternalImpl {
    pub fn new() -> ExternalImpl {
        ExternalImpl{}
    }
}

impl External for ExternalImpl {
}
