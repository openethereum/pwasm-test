//! Module for linking test externals
use std::any::Any;
use std::cell::{RefCell, Ref};
use std::slice;
use std::ptr;

use pwasm_std::types::{H256, U256, Address};
use external::{External, ExternalInstance};

thread_local!(#[doc(hidden)] pub static EXTERNAL: RefCell<Box<External>> = RefCell::new(Box::new(ExternalInstance::default())));

#[doc(hidden)]
/// Set handling external for the current thread
/// Ideally should be done before each test to avoid dirty state
/// Macro `test_with_external` uses this function and can help with such setup
pub fn set_external(ext: Box<External>) {
	EXTERNAL.with(|e| {
		*e.borrow_mut() = ext;
	});
}

#[doc(hidden)]
pub fn get_external<T: External + Clone + 'static>() -> T {
	EXTERNAL.with(|arg| {
		let ref_cell: &RefCell<Box<External>> = arg;
		let ref_: Ref<Box<External>> = ref_cell.borrow();

		let any: &Any = ref_.as_any();
		let downcasted: &T = any.downcast_ref().unwrap();
		downcasted.clone()
	})
}
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn storage_read(key: *const u8, dst: *mut u8) {
	EXTERNAL.with(|r| {
		let key = slice::from_raw_parts(key, 32);
		let result = r.borrow_mut().storage_read(&H256::from_slice(key));
		ptr::copy(result.as_ptr(), dst, result.len());
	})
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn storage_write(key: *const u8, src_raw: *const u8) {
	EXTERNAL.with(|r| {
		let key = slice::from_raw_parts(key, 32);
		let mut src = [0u8; 32];
		let src_slice = slice::from_raw_parts(src_raw, 32);
		src.copy_from_slice(src_slice);
		r.borrow().storage_write(&H256::from_slice(key), &src);
	})
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn create(endowment_ptr: *const u8, code_ptr: *const u8, code_len: u32, address_ptr: *mut u8) -> i32 {
	EXTERNAL.with(|r| {
		let endowment = U256::from_big_endian(slice::from_raw_parts(endowment_ptr, 32));
		let code: &[u8] = slice::from_raw_parts(code_ptr, code_len as usize);
		match r.borrow().create(endowment, code) {
			Ok(result) => { ptr::copy(result.as_ptr(), address_ptr, Address::len_bytes()); 0 },
			Err(_e) => 1
		}
	})
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ccall(
	gas: u64,
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
		match r.borrow().call(gas, &address, val, input, result) {
			Ok(_r) => 0,
			Err(_e) => 1
		}
	})
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn dcall(
	gas: u64,
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
		match r.borrow().call_code(gas, &address, input, result) {
			Ok(_r) => 0,
			Err(_e) => 1
		}
	})
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn scall(
	gas: u64,
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
		match r.borrow().call_code(gas, &address, input, result) {
			Ok(_r) => 0,
			Err(_e) => 1
		}
	})
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn suicide(refund_ptr: *const u8) {
	EXTERNAL.with(|r| {
		let address = Address::from_slice(slice::from_raw_parts(refund_ptr, 20));
		r.borrow().suicide(&address)
	})
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn blockhash(number: i64, dest: *mut u8) -> i32 {
	EXTERNAL.with(|r| {
		match r.borrow().blockhash(number as u64) {
			Ok(result) => { ptr::copy(result.as_ptr(), dest, Address::len_bytes()); 0 },
			Err(_e) => 1
		}
	})
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn coinbase(dest: *mut u8) {
	EXTERNAL.with(|r| {
		ptr::copy(r.borrow().coinbase().as_mut_ptr(), dest, 20);
	})
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn timestamp() -> i64 {
	EXTERNAL.with(|r| {
		r.borrow().timestamp() as i64
	})
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn blocknumber() -> i64 {
	EXTERNAL.with(|r| {
		r.borrow().blocknumber() as i64
	})
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn difficulty(dest: *mut u8) {
	let mut dest = slice::from_raw_parts_mut(dest, 32);
	EXTERNAL.with(|r| {
		r.borrow().difficulty().to_big_endian(&mut dest);
	});
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn gaslimit(dest: *mut u8) {
	let mut dest = slice::from_raw_parts_mut(dest, 32);
	EXTERNAL.with(|r| {
		r.borrow().gas_limit().to_big_endian(&mut dest);
	});
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn sender(dest: *mut u8) {
	EXTERNAL.with(|r| {
		ptr::copy(r.borrow().sender().as_ptr(), dest , 20);
	});
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn address(dest: *mut u8) {
	EXTERNAL.with(|r| {
		ptr::copy(r.borrow().address().as_ptr(), dest , 20);
	});
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn value(dest: *mut u8) {
	EXTERNAL.with(|r| {
		let mut dest = slice::from_raw_parts_mut(dest, 32);
		r.borrow().value().to_big_endian(&mut dest);
	})
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn origin(dest: *mut u8) {
	EXTERNAL.with(|r| {
		ptr::copy(r.borrow().origin().as_ptr(), dest , 20);
	});
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn balance(address_ptr: *const u8, balance_ptr: *mut u8) {
	EXTERNAL.with(|r| {
		let address = Address::from_slice(slice::from_raw_parts(address_ptr, 20));
		let mut balance =  slice::from_raw_parts_mut(balance_ptr, 32);
		r.borrow().balance(&address).to_big_endian(&mut balance);
	});
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn debug(str_ptr: *const u8, str_len: u32) {
	EXTERNAL.with(|r| {
		let msg = String::from_raw_parts(str_ptr as *mut _, str_len as usize, str_len as usize);
		r.borrow().debug_log(msg);
	});
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn elog(topic_ptr: *const u8, topic_count: u32, data_ptr: *const u8, data_len: u32) {
	EXTERNAL.with(|r| {
		let topics: &[H256] = slice::from_raw_parts(topic_ptr as *const H256, topic_count as usize);
		let data: &[u8] = slice::from_raw_parts(data_ptr, data_len as usize);
		r.borrow().elog(topics, data);
	});
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ret(_: *const u8, _: u32) -> ! {
	unimplemented!()
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn input_length() -> u32 {
	unimplemented!()
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn fetch_input(_: *mut u8) {
	unimplemented!()
}
