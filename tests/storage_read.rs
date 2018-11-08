extern crate pwasm_test;
extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_std::types::H256;
use pwasm_test::ext_reset;

#[test]
fn read_storage() {
	ext_reset(|e| e.storage(H256::zero(), [250; 32]));
	assert_eq!([250; 32], pwasm_ethereum::read(&H256::zero()));
}
