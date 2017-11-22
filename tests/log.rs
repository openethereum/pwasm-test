#[macro_use]
extern crate pwasm_test;
extern crate pwasm_std;

use pwasm_std::hash::H256;
use pwasm_std::ext;
use pwasm_test::{ExternalBuilder, ExternalInstance, get_external};

/// An example of how to use get_external to access log of some contract
test_with_external!(
	ExternalBuilder::new().build(),
	fetch_log {
		// Somewhere inside of the contract:
		let topics = [H256::new(), H256::new()];
		let data: &[u8] = b"some data";
		ext::log(&topics, data);

		let external = get_external::<ExternalInstance>();
		let log = external.logs();
		assert_eq!(log.len(), 1);
		let entry = &log[0];
		assert_eq!(entry.topics.as_ref(), &[H256::new(), H256::new()]);
		assert_eq!(entry.data.as_ref(), b"some data");
	}
);
