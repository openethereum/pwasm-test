extern crate pwasm_test;
extern crate pwasm_std;
extern crate pwasm_ethereum;
extern crate bigint;

use bigint::U256;
use pwasm_std::hash::Address;
use pwasm_test::ext_reset;

#[test]
fn externals() {
	ext_reset(|e| e
		.sender("0x16a0772b17ae004e6645e0e95bf50ad69498a34e".into())
		.address("0x35da6abcb08f2b6164fe380bb6c47bd8f2304d55".into())
		.origin("0x51f9c432a4e59ac86282d6adab4c2eb8919160eb".into())
		.coinbase("0xc257274276a4e539741ca11b590b9447b26a8051".into())
		.difficulty(123.into())
		.gas_left(1232133u64)
		.gas_limit(1234.into())
		.value(12345.into())
		.blocknumber(123123u64)
		.timestamp(123124u64)
	);
	assert_eq!(Address::from("0x16a0772b17ae004e6645e0e95bf50ad69498a34e"), pwasm_ethereum::sender());
	assert_eq!(Address::from("0x35da6abcb08f2b6164fe380bb6c47bd8f2304d55"), pwasm_ethereum::address());
	assert_eq!(Address::from("0x51f9c432a4e59ac86282d6adab4c2eb8919160eb"), pwasm_ethereum::origin());
	assert_eq!(Address::from("0xc257274276a4e539741ca11b590b9447b26a8051"), pwasm_ethereum::coinbase());
	assert_eq!(U256::from(123), pwasm_ethereum::difficulty());
	assert_eq!(1232133u64, pwasm_ethereum::gas_left());
	assert_eq!(U256::from(1234), pwasm_ethereum::gas_limit());
	assert_eq!(U256::from(12345), pwasm_ethereum::value());
	assert_eq!(123123u64, pwasm_ethereum::block_number());
	assert_eq!(123124u64, pwasm_ethereum::timestamp());
}
