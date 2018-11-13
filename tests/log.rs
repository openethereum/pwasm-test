extern crate pwasm_test;
extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_std::types::H256;
use pwasm_test::{ext_reset, ext_update, ext_get};

/// An example of how to use get_external to access log of some contract
#[test]
fn fetch_log() {
	ext_reset(|e| e);
	// Somewhere inside of the contract:
	let topics = [H256::zero(), H256::zero()];
	let data: &[u8] = b"some data";
	pwasm_ethereum::log(&topics, data);
	let log =  ext_get().logs();
	assert_eq!(log.len(), 1);
	let entry = &log[0];
	assert_eq!(entry.topics.as_ref(), &[H256::zero(), H256::zero()]);
	assert_eq!(entry.data.as_ref(), b"some data");
}

#[test]
fn log_update_ext() {
	ext_reset(|e| e);
	// Somewhere inside of the contract:
	let topics = [H256::zero(), H256::zero()];
	let data: &[u8] = b"some data";
	pwasm_ethereum::log(&topics, data);
	assert_eq!(ext_get().logs().len(), 1);
	ext_update(|e| e);
	pwasm_ethereum::log(&topics, data);
	assert_eq!(ext_get().logs().len(), 2);
}
