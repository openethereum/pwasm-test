use std::any::Any;
use pwasm_std::hash::{H256, Address};
use pwasm_std::bigint::U256;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Error;

pub type Endpoint = Box<FnMut(U256, &[u8], &mut [u8]) -> Result<(), Error>>;

/// Trait to manage calls to blockchain externs locally
pub trait External {

	/// Invoked when contract is requesting balance extern
	fn balance(&mut self, _address: &Address) -> U256 {
		unimplemented!()
	}

	/// Invoked when contract is requesting storage_read extern
	fn storage_read(&mut self, _key: &H256) -> [u8; 32]  {
		unimplemented!()
	}

	/// Invoked when contract is requesting storage_write extern
	fn storage_write(&mut self, _key: &H256, _value: &[u8; 32]) {
		unimplemented!()
	}

	/// Invoked when contract is requesting suicide extern
	fn suicide(&mut self, _refund: &Address) {
		unimplemented!()
	}

	/// Invoked when contract is requesting create extern
	fn create(&mut self, _endowment: U256, _code: &[u8]) -> Result<Address, Error> {
		unimplemented!()
	}

	/// Invoked when contract is requesting regular call (ccall) extern
	fn call(&mut self, _address: &Address, _val: U256, _input: &[u8], _result: &mut [u8]) -> Result<(), Error> {
		unimplemented!()
	}

	/// Invoked when contract is requesting delegate call (dcall) extern
	fn call_code(&mut self, _address: &Address, _input: &[u8], _result: &mut [u8]) -> Result<(), Error> {
		unimplemented!()
	}

	/// Invoked when contract is requesting static call (ccall) extern
	fn static_call(&mut self, _address: &Address, _input: &[u8], _result: &mut [u8]) -> Result<(), Error> {
		unimplemented!()
	}

	/// Invoked when contract fires an event
	fn log(&mut self, _topics: &[H256], _data: &[u8]) {
		unimplemented!()
	}

	/// Invoked when contract is requesting debug message extern
	fn debug_log(&mut self, _msg: String) {
		unimplemented!()
	}

	/// Invoked when contract is requesting blockhash extern
	fn blockhash(&mut self, _number: u64) -> Result<H256, Error> {
		unimplemented!()
	}

	/// Invoked when contract is requesting coinbase extern
	fn coinbase(&mut self) -> Address {
		unimplemented!()
	}

	/// Invoked when contract is requesting timestamp extern
	fn timestamp(&mut self) -> u64 {
		unimplemented!()
	}

	/// Invoked when contract is requesting blocknumber extern
	fn blocknumber(&mut self) -> u64 {
		unimplemented!()
	}

	/// Invoked when contract is requesting difficulty extern
	fn difficulty(&mut self) -> U256 {
		unimplemented!()
	}

	/// Invoked when contract is requesting gas_limit extern
	fn gas_limit(&mut self) -> U256 {
		unimplemented!()
	}

	/// Invoked when contract is requesting sender data
	fn sender(&mut self) -> Address {
		unimplemented!()
	}

	/// Invoked when contract is requesting origin data
	fn origin(&mut self) -> Address {
		unimplemented!()
	}

	/// Invoked when contract is requesting value data
	fn value(&mut self) -> U256 {
		unimplemented!()
	}

	/// Invoked when contract is requesting contract address
	fn address(&mut self) -> Address {
		unimplemented!()
	}

	fn as_any(&self) -> &Any;
}

#[derive(Clone, Default)]
pub struct Call {
	pub address: Address,
	pub value: U256,
	pub input: Box<[u8]>
}

#[derive(Clone, Default)]
pub struct LogEntry {
	pub topics: Box<[H256]>,
	pub data: Box<[u8]>,
}

#[derive(Clone, Default)]
pub struct ExternalInstance {
	pub storage: HashMap<H256, [u8; 32]>,
	pub balances: HashMap<Address, U256>,
	pub endpoints: HashMap<Address, Rc<Endpoint>>,
	pub calls: Vec<Call>,
	pub log: Vec<LogEntry>,
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
	pub fn calls(&self) -> Vec<Call> {
		self.calls.clone()
	}
	pub fn logs(&self) -> Vec<LogEntry> {
		self.log.clone()
	}
}

impl External for ExternalInstance {
	fn storage_read(&mut self, key: &H256) -> [u8; 32] {
		if let Some(value) = self.storage.get(key) {
			value.clone()
		} else {
			[0u8; 32]
		}
	}

	fn balance(&mut self, address: &Address) -> U256 {
		self.balances[address]
	}

	fn storage_write(&mut self, key: &H256, value: &[u8; 32]) {
		self.storage.insert(*key, value.clone());
	}

	fn call(&mut self, address: &Address, val: U256, input: &[u8], result: &mut [u8]) -> Result<(), Error> {
		self.calls.push(Call {
			address: address.clone(),
			value: val,
			input: Box::from(input)
		});
		if let Some(endpoint) = self.endpoints.get_mut(address) {
			Rc::get_mut(endpoint).unwrap()(val, input, result)
		} else {
			Ok(())
		}
	}

	fn log(&mut self, topics: &[H256], data: &[u8]) {
		self.log.push(LogEntry {
			topics: Box::from(topics),
			data: Box::from(data)
			}
		);
	}

	fn sender(&mut self) -> Address {
		self.sender
	}

	fn coinbase(&mut self) -> Address {
		self.coinbase
	}

	fn timestamp(&mut self) -> u64 {
		self.timestamp
	}

	fn blocknumber(&mut self) -> u64 {
		self.blocknumber
	}

	fn difficulty(&mut self) -> U256 {
		self.difficulty
	}

	fn gas_limit(&mut self) -> U256 {
		self.gas_limit
	}

	fn origin(&mut self) -> Address {
		self.origin
	}

	fn value(&mut self) -> U256 {
		self.value
	}

	fn address(&mut self) -> Address {
		self.address
	}

	fn as_any(&self) -> &Any {
		self
	}
}
