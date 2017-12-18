#[macro_use]
extern crate pwasm_test;
extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_std::hash::H256;
use pwasm_ethereum::storage;

use pwasm_test::ExternalBuilder;

test_with_external!(
	ExternalBuilder::new().storage(H256::new(), [250; 32]).build(),
	read_storage {
		assert_eq!([250; 32], storage::read(&H256::new()));
	}
);
