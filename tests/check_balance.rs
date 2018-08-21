extern crate pwasm_test;
extern crate pwasm_std;
extern crate pwasm_ethereum;
extern crate uint;

use uint::U256;
use pwasm_std::hash::Address;

use pwasm_test::ext_reset;

#[test]
fn check_balance() {
	ext_reset(|e| e.balance_of(Address::from([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]), 200000.into()));
	assert_eq!(
		U256::from(200000),
		pwasm_ethereum::balance(&Address::from([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20])));
}
