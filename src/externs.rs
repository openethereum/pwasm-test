//! Module for linking test externals

use std::cell::RefCell;
use std::slice;
use std::ptr;

use pwasm_std::hash::{H256, Address};
use pwasm_std::bigint::U256;

use external::{External, ExternalImpl};

thread_local!(pub static EXTERNAL: RefCell<Box <External>> = RefCell::new(Box::new(ExternalImpl)));

/// Set handling external for the current thread
/// Ideally should be done before each test to avoid dirty state
/// Macro `test_with_external` uses this function and can help with such setup
pub fn set_external(ext: Box<External>) {
	EXTERNAL.with(|e| {
		*e.borrow_mut() = ext;
	});
}

#[no_mangle]
pub unsafe extern "C" fn storage_read(key: *const u8, dst: *mut u8) -> i32 {
	EXTERNAL.with(|r| {
		let key = slice::from_raw_parts(key, 32);
		match r.borrow_mut().storage_read(&H256::from_slice(key)) {
			Ok(result) => { ptr::copy(result.as_ptr(), dst, result.len()); 0 },
			Err(_e) => 1
		}
	})
}

#[no_mangle]
pub unsafe extern "C" fn storage_write(key: *const u8, src_raw: *const u8) -> i32 {
	EXTERNAL.with(|r| {
		let key = slice::from_raw_parts(key, 32);
		let mut src = [0u8; 32];
		let src_slice = slice::from_raw_parts(src_raw, 32);
		src.copy_from_slice(src_slice);
		match r.borrow_mut().storage_write(&H256::from_slice(key), &src) {
			Ok(_r) => 0,
			Err(_e) => 1
		}
	})
}

#[no_mangle]
pub unsafe extern "C" fn create(endowment_ptr: *const u8, code_ptr: *const u8, code_len: u32, address_ptr: *mut u8) -> i32 {
	EXTERNAL.with(|r| {
		let endowment = U256::from_big_endian(slice::from_raw_parts(endowment_ptr, 32));
		let code: &[u8] = slice::from_raw_parts(code_ptr, code_len as usize);
		match r.borrow_mut().create(endowment, code) {
			Ok(result) => { ptr::copy(result.as_ptr(), address_ptr, result.len()); 0 },
			Err(_e) => 1
		}
	})
}

#[no_mangle]
pub unsafe extern "C" fn ccall(
	address_ptr: *const u8,
	val_ptr: *const u8,
	input_ptr: *const u8,
	input_len: u32,
	result_ptr: *mut u8,
	result_len: u32,
) -> i32
{
	EXTERNAL.with(|r| {
		let address = Address::from_slice(slice::from_raw_parts(address_ptr, 20));
		let val = U256::from_big_endian(slice::from_raw_parts(val_ptr, 32));
		let input: &[u8] = slice::from_raw_parts(input_ptr, input_len as usize);
		let result: &mut[u8] = slice::from_raw_parts_mut(result_ptr, result_len as usize);
		match r.borrow_mut().call(&address, val, input, result) {
			Ok(_r) => 0,
			Err(_e) => 1
		}
	})
}

#[no_mangle]
pub unsafe extern "C" fn dcall(
	address_ptr: *const u8,
	input_ptr: *const u8,
	input_len: u32,
	result_ptr: *mut u8,
	result_len: u32,
) -> i32
{
	EXTERNAL.with(|r| {
		let address = Address::from_slice(slice::from_raw_parts(address_ptr, 20));
		let input: &[u8] = slice::from_raw_parts(input_ptr, input_len as usize);
		let result: &mut[u8] = slice::from_raw_parts_mut(result_ptr, result_len as usize);
		match r.borrow_mut().call_code(&address, input, result) {
			Ok(_r) => 0,
			Err(_e) => 1
		}
	})
}

#[no_mangle]
pub unsafe extern "C" fn scall(
	address_ptr: *const u8,
	input_ptr: *const u8,
	input_len: u32,
	result_ptr: *mut u8,
	result_len: u32,
) -> i32
{
	EXTERNAL.with(|r| {
		let address = Address::from_slice(slice::from_raw_parts(address_ptr, 20));
		let input: &[u8] = slice::from_raw_parts(input_ptr, input_len as usize);
		let result: &mut[u8] = slice::from_raw_parts_mut(result_ptr, result_len as usize);
		match r.borrow_mut().call_code(&address, input, result) {
			Ok(_r) => 0,
			Err(_e) => 1
		}
	})
}

#[no_mangle]
pub unsafe extern "C" fn suicide(refund_ptr: *const u8) {
	EXTERNAL.with(|r| {
		let address = Address::from_slice(slice::from_raw_parts(refund_ptr, 20));
		r.borrow_mut().suicide(&address)
	})
}

#[no_mangle]
pub unsafe extern "C" fn blockhash(number: i64, dest: *mut u8) -> i32 {
	EXTERNAL.with(|r| {
		match r.borrow_mut().blockhash(number as u64) {
			Ok(mut result) => { ptr::copy(result.as_ptr(), dest, result.len()); 0 },
			Err(_e) => 1
		}
	})
}

#[no_mangle]
pub unsafe extern "C" fn coinbase(dest: *mut u8) {
	EXTERNAL.with(|r| {
		ptr::copy(dest, r.borrow_mut().coinbase().as_mut_ptr(), 20);
	})
}

#[no_mangle]
pub unsafe extern "C" fn timestamp() -> i64 {
	EXTERNAL.with(|r| {
		r.borrow_mut().timestamp() as i64
	})
}

#[no_mangle]
pub unsafe extern "C" fn blocknumber() -> i64 {
	EXTERNAL.with(|r| {
		r.borrow_mut().blocknumber() as i64
	})
}

#[no_mangle]
pub unsafe extern "C" fn difficulty(dest: *mut u8) {
	let mut dest = slice::from_raw_parts_mut(dest, 32);
	EXTERNAL.with(|r| {
		r.borrow_mut().difficulty().to_big_endian(&mut dest);
	});
}

#[no_mangle]
pub unsafe extern "C" fn gaslimit(dest: *mut u8) {
	let mut dest = slice::from_raw_parts_mut(dest, 32);
	EXTERNAL.with(|r| {
		r.borrow_mut().difficulty().to_big_endian(&mut dest);
	});
}

#[no_mangle]
pub unsafe extern "C" fn sender(dest: *mut u8) {
	EXTERNAL.with(|r| {
		ptr::copy(r.borrow_mut().sender().as_ptr(), dest , 20);
	});
}

#[no_mangle]
pub unsafe extern "C" fn address(dest: *mut u8) {
	EXTERNAL.with(|r| {
		ptr::copy(r.borrow_mut().address().as_ptr(), dest , 20);
	});
}

#[no_mangle]
pub unsafe extern "C" fn value(dest: *mut u8) {
	EXTERNAL.with(|r| {
		let mut dest = slice::from_raw_parts_mut(dest, 32);
		r.borrow_mut().value().to_big_endian(&mut dest);
	})
}

#[no_mangle]
pub unsafe extern "C" fn origin(dest: *mut u8) {
	EXTERNAL.with(|r| {
		ptr::copy(r.borrow_mut().origin().as_ptr(), dest , 20);
	});
}

#[no_mangle]
pub unsafe extern "C" fn balance(address_ptr: *const u8, balance_ptr: *mut u8) {
	EXTERNAL.with(|r| {
		let address = Address::from_slice(slice::from_raw_parts(address_ptr, 20));
		let mut balance =  slice::from_raw_parts_mut(balance_ptr, 32);
		r.borrow_mut().balance(&address).to_big_endian(&mut balance);
	});
}


#[no_mangle]
pub unsafe extern "C" fn debug(str_ptr: *const u8, str_len: u32) {
	EXTERNAL.with(|r| {
		let msg = String::from_raw_parts(str_ptr as *mut _, str_len as usize, str_len as usize);
		r.borrow_mut().debug_log(msg);
	});
}
