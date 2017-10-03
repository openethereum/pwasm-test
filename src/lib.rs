extern crate parity_hash;
extern crate bigint;

use std::cell::RefCell;
use std::thread;
use std::slice;
use std::borrow::Borrow;
use std::mem;
use std::ptr;

use parity_hash::{ Address, H256 };
use bigint::{ U256 };

pub mod external;

pub use external::External;
use external::{ ExternalImpl };


thread_local!(pub static external: RefCell<Box <External>> = RefCell::new(Box::new(ExternalImpl::new())));

pub fn set_external(ext: Box<External>) {
    external.with(|e| {
        *e.borrow_mut() = ext;
    });
}

#[macro_export]
macro_rules! test_with_external {
    (
       $struc:ident: $imp:item $($test_name:ident $test_body:block)*
    ) => {
        struct $struc;
        impl $struc {
            fn new() -> $struc {
                $struc{}
            }
        }
        $imp

        $(#[test]
        fn $test_name() {
            $crate::set_external(Box::new($struc::new()));
            $test_body
        })*
    }
}

#[no_mangle]
pub extern fn storage_read(key: *const u8, dst: *mut u8) -> i32 {
    external.with(|r| {
        let key = unsafe { slice::from_raw_parts(key, 32) };
        let dst = unsafe { slice::from_raw_parts_mut(dst, 32) };
        r.borrow_mut().storage_read(&H256::from_slice(key), dst)
    })
}

#[no_mangle]
pub extern fn storage_write(key: *const u8, src: *const u8) -> i32 {
    external.with(|r| {
        let key = unsafe { slice::from_raw_parts(key, 32) };
        let src = unsafe { slice::from_raw_parts(src, 32) };
        r.borrow_mut().storage_write(&H256::from_slice(key), src)
    })
}

#[no_mangle]
pub extern fn create(endowment_ptr: *const u8, code_ptr: *const u8, code_len: u32, address_ptr: *mut u8) -> i32 {
    external.with(|r| {
            let endowment = unsafe { U256::from_big_endian(slice::from_raw_parts(endowment_ptr, 32)) };
            let code: &[u8] = unsafe { slice::from_raw_parts(code_ptr, code_len as usize)};
            match r.borrow_mut().create(endowment, code) {
                Ok(result) => { unsafe { ptr::copy(result.as_ptr(), address_ptr, result.len()) }; 0 },
                Err(_e) => 1
            }
        })
}

#[no_mangle]
pub extern fn ccall(
            address_ptr: *const u8,
            val_ptr: *const u8,
            input_ptr: *const u8,
            input_len: u32,
            result_ptr: *mut u8,
            result_len: u32,
        ) -> i32 {
    external.with(|r| {
            let address = unsafe { Address::from_slice(slice::from_raw_parts(address_ptr, 20)) };
            let val = unsafe { U256::from_big_endian(slice::from_raw_parts(val_ptr, 32)) };
            let input: &[u8] = unsafe { slice::from_raw_parts(input_ptr, input_len as usize)};
            let result: &mut[u8] = unsafe { slice::from_raw_parts_mut(result_ptr, result_len as usize)};
            r.borrow_mut().call(&address, val, input, result)
        })
}

#[no_mangle]
pub extern fn dcall(
            address_ptr: *const u8,
            val_ptr: *const u8,
            input_ptr: *const u8,
            input_len: u32,
            result_ptr: *mut u8,
            result_len: u32,
        ) -> i32 {
        external.with(|r| {
            let address = unsafe { Address::from_slice(slice::from_raw_parts(address_ptr, 20)) };
            let input: &[u8] = unsafe { slice::from_raw_parts(input_ptr, input_len as usize)};
            let result: &mut[u8] = unsafe { slice::from_raw_parts_mut(result_ptr, result_len as usize)};
            r.borrow_mut().call_code(&address, input, result)
        })
}

#[no_mangle]
pub extern fn scall(
            address_ptr: *const u8,
            input_ptr: *const u8,
            input_len: u32,
            result_ptr: *mut u8,
            result_len: u32,
        ) -> i32 {
    external.with(|r| {
            let address = unsafe { Address::from_slice(slice::from_raw_parts(address_ptr, 20)) };
            let input: &[u8] = unsafe { slice::from_raw_parts(input_ptr, input_len as usize)};
            let result: &mut[u8] = unsafe { slice::from_raw_parts_mut(result_ptr, result_len as usize)};
            r.borrow_mut().call_code(&address, input, result)
        })
}

#[no_mangle]
pub extern fn suicide(refund_ptr: *const u8) {
    external.with(|r| {
        let address = unsafe { Address::from_slice(slice::from_raw_parts(refund_ptr, 20)) };
        r.borrow_mut().suicide(&address)
    })
}

#[no_mangle]
pub extern fn blockhash(number: i64, dest: *mut u8) -> i32 {
    external.with(|r| {
        let dest = unsafe { slice::from_raw_parts_mut(dest, 32) };
        r.borrow_mut().blockhash(number, dest)
    })
}

#[no_mangle]
pub extern fn coinbase(dest: *mut u8) {
    // external.with(|r| {
    //     let dest = unsafe { slice::from_raw_parts_mut(dest, 32) };
    //     r.borrow_mut().coinbase(dest);
    // })
}

#[no_mangle]
pub extern fn timestamp() -> i64 {
    1
}

#[no_mangle]
pub extern fn blocknumber() -> i64 {
    1
}

#[no_mangle]
pub extern fn difficulty(dest: *mut u8) {

}

#[no_mangle]
pub extern fn gaslimit(dest: *mut u8) {

}

#[no_mangle]
pub extern fn sender(dest: *mut u8) {

}

#[no_mangle]
pub extern fn address(dest: *mut u8) {

}

#[no_mangle]
pub extern fn value(dest: *mut u8) {
    external.with(|r| {
        let mut dest = unsafe { slice::from_raw_parts_mut(dest, 32) };
        r.borrow_mut().value().to_big_endian(&mut dest);
    })
}

#[no_mangle]
pub extern fn origin(dest: *mut u8) {

}

#[no_mangle]
pub extern fn debug(str_ptr: *const u8, str_len: u32) {

}
