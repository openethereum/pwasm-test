extern crate pwasm_test;
extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_std::hash::Address;

use pwasm_test::{ext_reset, ext_update, ext_get, Endpoint};

/// An example of how to use get_external to access "calls" to some contract
#[test]
fn has_called() {
	ext_reset(|e| e);
	let mut result = [0u8; 1];
	let input = [2u8; 32];
	pwasm_ethereum::call(2000, &Address::new(), 10000.into(), &input, &mut result).expect_err("Should be an Error");
	let calls = ext_get().calls();
	assert_eq!(calls.len(), 1);

	let call = &calls[0];
	assert_eq!(call.value, 10000.into());
	assert_eq!(call.address, Address::new());
	assert!(&input == call.input.as_ref());
}

#[test]
fn endpoint_has_called () {
	ext_reset(|e| e
		.endpoint("0x16a0772b17ae004e6645e0e95bf50ad69498a34e".into(), Endpoint::new(Box::new(|_val, _input, result| {
			result[0] = 2;
			Ok(())
		})))
	);
	let mut result = [0u8; 1];
	let input = [2u8; 32];
	pwasm_ethereum::call(20000, &"0x16a0772b17ae004e6645e0e95bf50ad69498a34e".into(), 100.into(), &input, &mut result).unwrap();
	assert_eq!(result[0], 2);
}

#[test]
fn endpoint_call_each_other () {
	ext_reset(|e| e
		.endpoint("0x16a0772b17ae004e6645e0e95bf50ad69498a34e".into(), Endpoint::new(Box::new(|val, input,  mut result| {
			result[0] = 2;
			pwasm_ethereum::call(val.into(), &"0x35da6abcb08f2b6164fe380bb6c47bd8f2304d55".into(), 100.into(), &input, &mut result).unwrap();
			Ok(())
		})))
		.endpoint("0x35da6abcb08f2b6164fe380bb6c47bd8f2304d55".into(), Endpoint::new(Box::new(|_val, _input, result| {
			result[0] = 3;
			Ok(())
		})))
	);
	let mut result = [0u8; 1];
	let input = [2u8; 32];
	pwasm_ethereum::call(20000, &"0x16a0772b17ae004e6645e0e95bf50ad69498a34e".into(), 100.into(), &input, &mut result).unwrap();
	assert_eq!(result[0], 3);
}

#[test]
fn calls_update_ext() {
	ext_reset(|e| e);
	let mut result = [0u8; 1];
	let input = [2u8; 32];
	pwasm_ethereum::call(2000, &Address::new(), 10000.into(), &input, &mut result).expect_err("Should be an Error");
	assert_eq!(ext_get().calls().len(), 1);
	ext_update(|e| e);
	pwasm_ethereum::call(2000, &Address::new(), 10000.into(), &input, &mut result).expect_err("Should be an Error");
	assert_eq!(ext_get().calls().len(), 2);
}
