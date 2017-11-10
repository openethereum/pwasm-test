#[macro_use]
extern crate pwasm_test;
extern crate pwasm_std;

use pwasm_std::hash::Address;
use pwasm_std::ext;
use pwasm_test::{External, Error};

#[derive(Default)]
struct DummyExternal;

impl External for DummyExternal {
	fn sender(&mut self) -> Address {
		"0x16a0772b17ae004e6645e0e95bf50ad69498a34e".into()
	}
}

test_with_external!(
	DummyExternal::default(),
	get_sender {
		assert_eq!(Address::from("0x16a0772b17ae004e6645e0e95bf50ad69498a34e"), ext::sender());
	}
);
