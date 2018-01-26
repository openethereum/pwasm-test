#[macro_use]
extern crate pwasm_test;
extern crate pwasm_std;
extern crate pwasm_ethereum;
extern crate bigint;

use bigint::U256;
use pwasm_std::hash::Address;
use pwasm_ethereum as eth;

use pwasm_test::ExternalBuilder;

test_with_external!(
	ExternalBuilder::new()
		.sender("0x16a0772b17ae004e6645e0e95bf50ad69498a34e".into())
		.address("0x35da6abcb08f2b6164fe380bb6c47bd8f2304d55".into())
		.origin("0x51f9c432a4e59ac86282d6adab4c2eb8919160eb".into())
		.coinbase("0xc257274276a4e539741ca11b590b9447b26a8051".into())
		.difficulty(123.into())
		.gas_limit(1234.into())
		.value(12345.into())
		.blocknumber(123123u64)
		.timestamp(123124u64)
		.build(),
	sender {
		assert_eq!(Address::from("0x16a0772b17ae004e6645e0e95bf50ad69498a34e"), eth::sender());
	}
	address {
		assert_eq!(Address::from("0x35da6abcb08f2b6164fe380bb6c47bd8f2304d55"), eth::address());
	}
	origin {
		assert_eq!(Address::from("0x51f9c432a4e59ac86282d6adab4c2eb8919160eb"), eth::origin());
	}
	coinbase {
		assert_eq!(Address::from("0xc257274276a4e539741ca11b590b9447b26a8051"), eth::coinbase());
	}
	difficulty {
		assert_eq!(U256::from(123), eth::difficulty());
	}
	gas_limit {
		assert_eq!(U256::from(1234), eth::gas_limit());
	}
	value {
		assert_eq!(U256::from(12345), eth::value());
	}
	blocknumber {
		assert_eq!(123123u64, eth::block_number());
	}
	timestamp {
		assert_eq!(123124u64, eth::timestamp());
	}
);
