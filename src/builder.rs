use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use pwasm_std::types::{H256, U256, Address};
use external::{ExternalInstance, Endpoint};

/// A builder for quick creation of External impls for testing.
pub struct ExternalBuilder {
	storage: HashMap<H256, [u8; 32]>,
	balances: HashMap<Address, U256>,
	endpoints: HashMap<Address, Rc<RefCell<Endpoint>>>,
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
			endpoints: HashMap::new(),
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

	/// Sets Endpoint closure to prosess `pwasm_ethereum::call` to some Address
	///
	/// # Example
	/// ```
	/// # extern crate pwasm_test;
	/// # extern crate pwasm_ethereum;
	/// # use pwasm_test::ext_reset;
	///	# use pwasm_test::Endpoint;
	/// # fn main () {
	/// #
	///	ext_reset(|e| e
	///		.endpoint("16a0772b17ae004e6645e0e95bf50ad69498a34e".parse().unwrap(),
	///			Endpoint::new(Box::new(|_val, _input, result| {
	///				result[0] = 2;
	///				Ok(())
	///		})))
	/// );
	///	let mut result = [0u8; 1];
	///	let input = [2u8; 32];
	///	pwasm_ethereum::call(20000, &"16a0772b17ae004e6645e0e95bf50ad69498a34e".parse().unwrap(), 100.into(), &input, &mut result).unwrap();
	///	assert_eq!(result[0], 2);
	/// # }
	/// ```
	///
	pub fn endpoint(mut self, address: Address, endpoint: Endpoint) -> Self {
		self.endpoints.insert(address, Rc::new(RefCell::new(endpoint)));
		self
	}

	/// Sets storage value for some `key`
	///
	/// # Example
	/// ```
	/// # extern crate pwasm_test;
	/// # extern crate pwasm_ethereum;
	/// # extern crate pwasm_std;
	/// # use pwasm_std::types::H256;
	/// # use pwasm_test::ext_reset;
	/// # fn main () {
	/// #
	///	ext_reset(|e| e.storage(H256::zero(), [250; 32]));
	///	assert_eq!(pwasm_ethereum::read(&H256::zero()), [250; 32]);
	/// # }
	/// ```
	pub fn storage(mut self, key: H256, value: [u8; 32]) -> Self {
		self.storage.insert(key, value);
		self
	}

	/// Sets `pwasm_ethereum::balance()` for some address
	///
	/// # Example
	/// ```
	/// # extern crate pwasm_test;
	/// # extern crate pwasm_ethereum;
	/// # use pwasm_test::ext_reset;
	/// # fn main () {
	/// #
	///	ext_reset(|e| e.balance_of("16a0772b17ae004e6645e0e95bf50ad69498a34e".parse().unwrap(), 200000.into()));
	///	assert_eq!(
	///		pwasm_ethereum::balance(&"16a0772b17ae004e6645e0e95bf50ad69498a34e".parse().unwrap()),
	///		200000.into());
	/// # }
	/// ```
	pub fn balance_of(mut self, key: Address, value: U256) -> Self {
		self.balances.insert(key, value);
		self
	}

	/// Sets `pwasm_ethereum::sender()`
	///
	/// # Example
	/// ```
	/// # extern crate pwasm_test;
	/// # extern crate pwasm_ethereum;
	/// # use pwasm_test::ext_reset;
	/// # fn main () {
	/// #
	///	ext_reset(|e| e.sender("16a0772b17ae004e6645e0e95bf50ad69498a34e".parse().unwrap()));
	///	pwasm_ethereum::sender();
	///	assert_eq!(pwasm_ethereum::sender(), "16a0772b17ae004e6645e0e95bf50ad69498a34e".parse().unwrap());
	/// # }
	/// ```
	pub fn sender(mut self, sender: Address) -> Self {
		self.sender = sender;
		self
	}


	/// Sets `pwasm_ethereum::coinbase()`
	///
	/// # Example
	/// ```
	/// # extern crate pwasm_test;
	/// # extern crate pwasm_ethereum;
	/// # use pwasm_test::ext_reset;
	/// # fn main () {
	/// #
	///	ext_reset(|e| e.coinbase("16a0772b17ae004e6645e0e95bf50ad69498a34e".parse().unwrap()));
	///	assert_eq!(
	///		pwasm_ethereum::coinbase(), "16a0772b17ae004e6645e0e95bf50ad69498a34e".parse().unwrap());
	/// # }
	/// ```
	pub fn coinbase(mut self, coinbase: Address) -> Self {
		self.coinbase = coinbase;
		self
	}


	/// Sets `pwasm_ethereum::timestamp()`
	///
	/// # Example
	/// ```
	/// # extern crate pwasm_test;
	/// # extern crate pwasm_ethereum;
	/// # use pwasm_test::ext_reset;
	/// # fn main () {
	/// #
	///	ext_reset(|e| e.timestamp(123124u64));
	///	assert_eq!(pwasm_ethereum::timestamp(), 123124u64);
	/// # }
	/// ```
	pub fn timestamp(mut self, timestamp: u64) -> Self {
		self.timestamp = timestamp;
		self
	}

	/// Sets `pwasm_ethereum::block_number()`
	///
	/// # Example
	/// ```
	/// # extern crate pwasm_test;
	/// # extern crate pwasm_ethereum;
	/// # use pwasm_test::ext_reset;
	/// # fn main () {
	/// #
	///	ext_reset(|e| e.blocknumber(123124u64));
	///	assert_eq!(pwasm_ethereum::block_number(), 123124u64);
	/// # }
	/// ```
	pub fn blocknumber(mut self, blocknumber: u64) -> Self {
		self.blocknumber = blocknumber;
		self
	}

	/// Sets `pwasm_ethereum::difficulty()`
	///
	/// # Example
	/// ```
	/// # extern crate pwasm_test;
	/// # extern crate pwasm_ethereum;
	/// # use pwasm_test::ext_reset;
	/// # fn main () {
	/// #
	///	ext_reset(|e| e.difficulty(123.into()));
	///	assert_eq!(pwasm_ethereum::difficulty(), 123.into());
	/// # }
	/// ```
	pub fn difficulty(mut self, difficulty: U256) -> Self {
		self.difficulty =  difficulty;
		self
	}

	/// Sets `pwasm_ethereum::gas_limit()`
	///
	/// # Example
	/// ```
	/// # extern crate pwasm_test;
	/// # extern crate pwasm_ethereum;
	/// # use pwasm_test::ext_reset;
	/// # fn main () {
	/// #
	///	ext_reset(|e| e.gas_limit(1337.into()));
	///	assert_eq!(pwasm_ethereum::gas_limit(), 1337.into());
	/// # }
	/// ```
	pub fn gas_limit(mut self, gas_limit: U256) -> Self {
		self.gas_limit = gas_limit;
		self
	}

	/// Sets `pwasm_ethereum::origin()`
	///
	/// # Example
	/// ```
	/// # extern crate pwasm_test;
	/// # extern crate pwasm_ethereum;
	/// # use pwasm_test::ext_reset;
	/// # fn main () {
	/// #
	///	ext_reset(|e| e.origin("51f9c432a4e59ac86282d6adab4c2eb8919160eb".parse().unwrap()));
	///	assert_eq!(pwasm_ethereum::origin(), "51f9c432a4e59ac86282d6adab4c2eb8919160eb".parse().unwrap());
	/// # }
	/// ```
	pub fn origin(mut self, origin: Address) -> Self {
		self.origin = origin;
		self
	}

	/// Sets `pwasm_ethereum::value()`
	///
	/// # Example
	/// ```
	/// # extern crate pwasm_test;
	/// # extern crate pwasm_ethereum;
	/// # use pwasm_test::ext_reset;
	/// # fn main () {
	/// #
	///	ext_reset(|e| e.value(999999.into()));
	///	assert_eq!(pwasm_ethereum::value(), 999999.into());
	/// # }
	/// ```
	pub fn value(mut self, value: U256) -> Self {
		self.value = value;
		self
	}

	/// Sets `pwasm_ethereum::address()`
	///
	/// # Example
	/// ```
	/// # extern crate pwasm_test;
	/// # extern crate pwasm_ethereum;
	/// #
	/// # use pwasm_test::ext_reset;
	/// # fn main () {
	/// #
	///	ext_reset(|e| e.address("35da6abcb08f2b6164fe380bb6c47bd8f2304d55".parse().unwrap()));
	///	assert_eq!(pwasm_ethereum::address(), "35da6abcb08f2b6164fe380bb6c47bd8f2304d55".parse().unwrap());
	/// # }
	/// ```
	pub fn address(mut self, address: Address) -> Self {
		self.address = address;
		self
	}

	/// Builds ExternalInstance from ExternalBuilder
	pub fn build(self) -> ExternalInstance {
		ExternalInstance {
			log: RefCell::new(Vec::new()),
			calls: RefCell::new(Vec::new()),
			storage: RefCell::new(self.storage),
			endpoints: self.endpoints,
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

	/// Restores ExternalBuilder from ExternalInstance
	pub fn from_instance(instance: ExternalInstance) -> ExternalBuilder {
		ExternalBuilder {
			endpoints: instance.endpoints.clone(),
			storage: instance.storage.borrow().clone(),
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
