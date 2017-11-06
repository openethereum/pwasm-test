#[macro_use]
extern crate pwasm_test;
extern crate pwasm_std;

use pwasm_std::hash::H256;
use pwasm_std::storage;
use pwasm_test::{External, Error};

#[derive(Default)]
struct DummyExternal;

impl External for DummyExternal {
	fn storage_read(&mut self, _key: &H256) -> Result<[u8; 32], Error> {
		Ok([250; 32])
	}
}

test_with_external!(
	DummyExternal::default(),
	read_storage {
		assert_eq!([250; 32], storage::read(&H256::new()).unwrap());
	}
);
