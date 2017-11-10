#[macro_use]
extern crate pwasm_test;
extern crate pwasm_std;

use pwasm_std::hash::Address;
use pwasm_std::bigint::U256;
use pwasm_std::ext;
use pwasm_test::{External};

#[derive(Default)]
struct DummyExternal;

impl External for DummyExternal {
	fn sender(&mut self) -> Address {
		"0x16a0772b17ae004e6645e0e95bf50ad69498a34e".into()
	}
	fn address(&mut self) -> Address {
		"0x35da6abcb08f2b6164fe380bb6c47bd8f2304d55".into()
	}
	fn origin(&mut self) -> Address {
		"0x51f9c432a4e59ac86282d6adab4c2eb8919160eb".into()
	}
	fn coinbase(&mut self) -> Address {
		"0xc257274276a4e539741ca11b590b9447b26a8051".into()
	}
	fn difficulty(&mut self) -> U256 {
		123.into()
	}
	fn gas_limit(&mut self) -> U256 {
		1234.into()
	}
	fn value(&mut self) -> U256 {
		12345.into()
	}
	fn blocknumber(&mut self) -> u64 {
		123123u64
	}
	fn timestamp(&mut self) -> u64 {
		123124u64
	}
}

test_with_external!(
	DummyExternal::default(),
	sender {
		assert_eq!(Address::from("0x16a0772b17ae004e6645e0e95bf50ad69498a34e"), ext::sender());
	}
	address {
		assert_eq!(Address::from("0x35da6abcb08f2b6164fe380bb6c47bd8f2304d55"), ext::address());
	}
	origin {
		assert_eq!(Address::from("0x51f9c432a4e59ac86282d6adab4c2eb8919160eb"), ext::origin());
	}
	coinbase {
		assert_eq!(Address::from("0xc257274276a4e539741ca11b590b9447b26a8051"), ext::coinbase());
	}
	difficulty {
		assert_eq!(U256::from(123), ext::difficulty());
	}
	gas_limit {
		assert_eq!(U256::from(1234), ext::gas_limit());
	}
	value {
		assert_eq!(U256::from(12345), ext::value());
	}
	blocknumber {
		assert_eq!(123123u64, ext::block_number());
	}
	timestamp {
		assert_eq!(123124u64, ext::timestamp());
	}
);
