use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;
use std::ops::DerefMut;
use std::cell::RefCell;

use pwasm_std::types::{H256, U256, Address};
use pwasm_abi::eth::EndpointInterface;

#[doc(hidden)]
#[derive(Debug)]
pub struct Error;

/// A fake contract endpoint
/// Endpoint is just a closure which receives `value: U256`, `input: &[u8]`, `output: &mut [u8]`
/// and returns `Ok(())` if call was successfull or `Err(Error)` otherwise
pub struct Endpoint (Box<FnMut(U256, &[u8], &mut [u8]) -> Result<(), Error>>);

impl Endpoint {
	pub fn new(f: Box<FnMut(U256, &[u8], &mut [u8]) -> Result<(), Error>>) -> Endpoint {
		Endpoint(f)
	}
	pub fn ok() -> Endpoint {
		Endpoint(Box::new(move |_, _, _| {
            Ok(())
        }))
	}
	pub fn err() -> Endpoint {
		Endpoint(Box::new(move |_, _, _| {
            Err(Error)
        }))
	}
}

/// Wraps any `pwasm_abi::eth::EndpointInterface` to `Endpoint`
impl<T: EndpointInterface + 'static> From<T> for Endpoint {
	fn from(mut intf: T) -> Endpoint {
        Endpoint(Box::new(move |_val, input, result| {
            result.copy_from_slice(&intf.dispatch(input));
            Ok(())
        }))
	}
}

#[doc(hidden)]
/// Trait to manage calls to blockchain externs locally
/// This trait methods are called by `pwasm_ethereum::*` externs, see `externs.rs`
pub trait External {

	/// Invoked when contract is calling `pwasm_ethereum::balance`
	fn balance(&self, _address: &Address) -> U256 {
		unimplemented!()
	}

	/// Invoked when contract is calling `pwasm_ethereum::read`
	fn storage_read(&self, _key: &H256) -> [u8; 32]  {
		unimplemented!()
	}

	/// Invoked when contract is calling `pwasm_ethereum::write`
	fn storage_write(&self, _key: &H256, _value: &[u8; 32]) {
		unimplemented!()
	}

	/// Invoked when contract is calling `pwasm_ethereum::suicide`
	fn suicide(&self, _refund: &Address) {
		unimplemented!()
	}

	/// Invoked when contract is calling `pwasm_ethereum::create`
	fn create(&self, _endowment: U256, _code: &[u8]) -> Result<Address, Error> {
		unimplemented!()
	}

	/// Invoked when contract is calling regular `pwasm_ethereum::ccall`
	fn call(&self, _gas: u64, _address: &Address, _val: U256, _input: &[u8], _result: &mut [u8]) -> Result<(), Error> {
		unimplemented!()
	}

	/// Invoked when contract is calling delegate (`pwasm_ethereum::dcall`)
	fn call_code(&self, _gas: u64, _address: &Address, _input: &[u8], _result: &mut [u8]) -> Result<(), Error> {
		unimplemented!()
	}

	/// Invoked when contract is calling static call (`pwasm_ethereum::ccall`)
	fn static_call(&self, _gas: u64, _address: &Address, _input: &[u8], _result: &mut [u8]) -> Result<(), Error> {
		unimplemented!()
	}

	/// Invoked when contract fires an event (calls `pwasm_ethereum::elog`)
	fn elog(&self, _topics: &[H256], _data: &[u8]) {
		unimplemented!()
	}

	/// Invoked when contract is calling debug message
	fn debug_log(&self, _msg: String) {
		unimplemented!()
	}

	/// Invoked when contract is calling `pwasm_ethereum::blockhash`
	fn blockhash(&self, _number: u64) -> Result<H256, Error> {
		unimplemented!()
	}

	/// Invoked when contract is calling `pwasm_ethereum::coinbase`
	fn coinbase(&self) -> Address {
		unimplemented!()
	}

	/// Invoked when contract is calling `pwasm_ethereum::timestamp`
	fn timestamp(&self) -> u64 {
		unimplemented!()
	}

	/// Invoked when contract is calling `pwasm_ethereum::blocknumber`
	fn blocknumber(&self) -> u64 {
		unimplemented!()
	}

	/// Invoked when contract is calling `pwasm_ethereum::difficulty`
	fn difficulty(&self) -> U256 {
		unimplemented!()
	}

	/// Invoked when contract is calling `pwasm_ethereum::gas_limit`
	fn gas_limit(&self) -> U256 {
		unimplemented!()
	}

	/// Invoked when contract is calling `pwasm_ethereum::sender`
	fn sender(&self) -> Address {
		unimplemented!()
	}

	/// Invoked when contract is calling `pwasm_ethereum::origin`
	fn origin(&self) -> Address {
		unimplemented!()
	}

	/// Invoked when contract is calling `pwasm_ethereum::value`
	fn value(&self) -> U256 {
		unimplemented!()
	}

	/// Invoked when contract is calling `pwasm_ethereum::address`
	fn address(&self) -> Address {
		unimplemented!()
	}

	fn as_any(&self) -> &Any;
}

#[derive(Clone, Default, Debug)]
pub struct Call {
	pub gas: u64,
	pub address: Address,
	pub value: U256,
	pub input: Box<[u8]>
}

#[derive(Clone, Default, Debug)]
pub struct LogEntry {
	pub topics: Box<[H256]>,
	pub data: Box<[u8]>,
}

#[doc(hidden)]
#[derive(Clone, Default)]
pub struct ExternalInstance {
	pub storage: RefCell<HashMap<H256, [u8; 32]>>,
	pub calls: RefCell<Vec<Call>>,
	pub log: RefCell<Vec<LogEntry>>,
	pub balances: HashMap<Address, U256>,
	pub endpoints: HashMap<Address, Rc<RefCell<Endpoint>>>,
	pub sender: Address,
	pub value: U256,
	pub address: Address,
	pub origin: Address,
	pub coinbase: Address,
	pub difficulty: U256,
	pub gas_limit: U256,
	pub blocknumber: u64,
	pub timestamp: u64,
}

impl ExternalInstance {

	/// Returns records of calls was done via `pwasm_ethereum::call` with address, value, gas, and provided input
	pub fn calls(&self) -> Vec<Call> {
		self.calls.borrow().clone()
	}
	/// Returns log entries added with `pwasm_ethereum::elog`
	pub fn logs(&self) -> Vec<LogEntry> {
		self.log.borrow().clone()
	}
}

impl External for ExternalInstance {
	fn storage_read(&self, key: &H256) -> [u8; 32] {
		if let Some(value) = self.storage.borrow().get(key) {
			value.clone()
		} else {
			[0u8; 32]
		}
	}

	fn balance(&self, address: &Address) -> U256 {
		self.balances[address]
	}

	fn storage_write(&self, key: &H256, value: &[u8; 32]) {
		self.storage.borrow_mut().insert(*key, value.clone());
	}

	fn call(&self, gas: u64, address: &Address, val: U256, input: &[u8], result: &mut [u8]) -> Result<(), Error> {
		self.calls.borrow_mut().push(Call {
			gas: gas,
			address: address.clone(),
			value: val,
			input: Box::from(input)
		});
		if let Some(endpoint) = self.endpoints.get(address) {
			endpoint.borrow_mut().deref_mut().0(val, input, result)
		} else {
			Err(Error)
		}
	}

	fn elog(&self, topics: &[H256], data: &[u8]) {
		self.log.borrow_mut().push(LogEntry {
			topics: Box::from(topics),
			data: Box::from(data)
			}
		);
	}

	fn sender(&self) -> Address {
		self.sender
	}

	fn coinbase(&self) -> Address {
		self.coinbase
	}

	fn timestamp(&self) -> u64 {
		self.timestamp
	}

	fn blocknumber(&self) -> u64 {
		self.blocknumber
	}

	fn difficulty(&self) -> U256 {
		self.difficulty
	}

	fn gas_limit(&self) -> U256 {
		self.gas_limit
	}

	fn origin(&self) -> Address {
		self.origin
	}

	fn value(&self) -> U256 {
		self.value
	}

	fn address(&self) -> Address {
		self.address
	}

	fn as_any(&self) -> &Any {
		self
	}
}
