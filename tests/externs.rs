extern crate pwasm_test;
extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_std::types::{U256, Address};
use pwasm_test::ext_reset;

#[test]
fn externals() {
	use std::str::FromStr;
	ext_reset(|e| e
		.sender("16a0772b17ae004e6645e0e95bf50ad69498a34e".parse().unwrap())
		.address("35da6abcb08f2b6164fe380bb6c47bd8f2304d55".parse().unwrap())
		.origin("51f9c432a4e59ac86282d6adab4c2eb8919160eb".parse().unwrap())
		.coinbase("c257274276a4e539741ca11b590b9447b26a8051".parse().unwrap())
		.difficulty(123.into())
		.gas_limit(1234.into())
		.value(12345.into())
		.blocknumber(123123u64)
		.timestamp(123124u64)
	);
	assert_eq!(Address::from_str("16a0772b17ae004e6645e0e95bf50ad69498a34e").unwrap(), pwasm_ethereum::sender());
	assert_eq!(Address::from_str("35da6abcb08f2b6164fe380bb6c47bd8f2304d55").unwrap(), pwasm_ethereum::address());
	assert_eq!(Address::from_str("51f9c432a4e59ac86282d6adab4c2eb8919160eb").unwrap(), pwasm_ethereum::origin());
	assert_eq!(Address::from_str("c257274276a4e539741ca11b590b9447b26a8051").unwrap(), pwasm_ethereum::coinbase());
	assert_eq!(U256::from(123), pwasm_ethereum::difficulty());
	assert_eq!(U256::from(1234), pwasm_ethereum::gas_limit());
	assert_eq!(U256::from(12345), pwasm_ethereum::value());
	assert_eq!(123123u64, pwasm_ethereum::block_number());
	assert_eq!(123124u64, pwasm_ethereum::timestamp());
}
