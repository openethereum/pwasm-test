use pwasm_std::hash::{H256, Address};
use pwasm_std::bigint::U256;
use std::collections::HashMap;

use external::ExternalInstance;

/// A builder for quick creation of External impls for testing.
pub struct ExternalBuilder {
	storage: HashMap<H256, [u8; 32]>,
	balances: HashMap<Address, U256>,
	value: U256,
	sender: Address,
	address: Address,
	origin: Address,
	coinbase: Address,
	difficulty: U256,
	gas_limit: U256,
	blocknumber: u64,
	timestamp: u64,
}

impl ExternalBuilder {
	/// Begin build process
	pub fn new() -> Self {
		ExternalBuilder {
			storage: HashMap::new(),
			sender: Address::default(),
			address: Address::default(),
			balances: HashMap::new(),
			value: U256::zero(),
			origin: Address::default(),
			coinbase: Address::default(),
			difficulty: U256::zero(),
			gas_limit: U256::zero(),
			blocknumber: 0u64,
			timestamp: 0u64,
		}
	}

	pub fn sender(mut self, sender: Address) -> Self {
		self.sender = sender;
		self
	}

	pub fn storage(mut self, key: H256, value: [u8; 32]) -> Self {
		self.storage.insert(key, value);
		self
	}

	pub fn balance_of(mut self, key: Address, value: U256) -> Self {
		self.balances.insert(key, value);
		self
	}

	pub fn coinbase(mut self, coinbase: Address) -> Self {
		self.coinbase = coinbase;
		self
	}

	pub fn timestamp(mut self, timestamp: u64) -> Self {
		self.timestamp = timestamp;
		self
	}

	pub fn blocknumber(mut self, blocknumber: u64) -> Self {
		self.blocknumber = blocknumber;
		self
	}

	pub fn difficulty(mut self, difficulty: U256) -> Self {
		self.difficulty =  difficulty;
		self
	}

	pub fn gas_limit(mut self, gas_limit: U256) -> Self {
		self.gas_limit = gas_limit;
		self
	}

	pub fn origin(mut self, origin: Address) -> Self {
		self.origin = origin;
		self
	}

	pub fn value(mut self, value: U256) -> Self {
		self.value = value;
		self
	}

	pub fn address(mut self, address: Address) -> Self {
		self.address = address;
		self
	}


	pub fn build(self) -> ExternalInstance {
		ExternalInstance {
			log: Vec::new(),
			calls: Vec::new(),
			storage: self.storage,
			balances: self.balances,
			sender: self.sender,
			value: self.value,
			origin: self.origin,
			address: self.address,
			coinbase: self.coinbase,
			difficulty: self.difficulty,
			gas_limit: self.gas_limit,
			blocknumber: self.blocknumber,
			timestamp: self.timestamp,
		}
	}

	pub fn from_instance(instance: ExternalInstance) -> ExternalBuilder {
		ExternalBuilder {
			storage: instance.storage,
			balances: instance.balances,
			sender: instance.sender,
			value: instance.value,
			origin: instance.origin,
			address: instance.address,
			coinbase: instance.coinbase,
			difficulty: instance.difficulty,
			gas_limit: instance.gas_limit,
			blocknumber: instance.blocknumber,
			timestamp: instance.timestamp,
		}
	}
}

impl From<ExternalInstance> for ExternalBuilder {
	fn from(instance: ExternalInstance) -> ExternalBuilder {
		ExternalBuilder::from_instance(instance)
	}
}
