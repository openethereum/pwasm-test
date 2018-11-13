//! `pwasm_ethereum` test lib
#![cfg_attr(not(feature = "std"), no_std)]

extern crate pwasm_std;
extern crate pwasm_ethereum;
extern crate pwasm_abi;

mod external;
mod externs;
mod builder;

pub use external::{Endpoint, External, ExternalInstance, Error};
pub use builder::ExternalBuilder;
pub use externs::*;

///	Allows to mock `pwasm_ethereum::*` calls
///
///	# Example
///
/// ```
/// extern crate pwasm_ethereum;
/// extern crate pwasm_test;
///
///	use pwasm_test::ext_reset;
///
/// fn main () {
///		ext_reset(|e| e.sender("16a0772b17ae004e6645e0e95bf50ad69498a34e".parse().unwrap()));
///		assert_eq!(pwasm_ethereum::sender(), "16a0772b17ae004e6645e0e95bf50ad69498a34e".parse().unwrap());
/// }
/// ```
///
pub fn ext_reset<F>(updater: F) where F: Fn(ExternalBuilder) -> ExternalBuilder {
	let ext = updater(ExternalBuilder::new()).build();
	set_external(Box::new(ext));
}

#[doc(inline)]
///	Updates `pwasm_ethereum::*` mocks
///
///	# Example
///
/// ```
/// extern crate pwasm_ethereum;
/// extern crate pwasm_test;
///
///	use pwasm_test::{ext_reset, ext_update};
///
/// fn main () {
///		ext_reset(|e| e.value(10000.into()));
///		ext_update(|e| e.sender("16a0772b17ae004e6645e0e95bf50ad69498a34e".parse().unwrap()));
///
/// 	assert_eq!(pwasm_ethereum::value(), 10000.into());
///		assert_eq!(pwasm_ethereum::sender(), "16a0772b17ae004e6645e0e95bf50ad69498a34e".parse().unwrap());
/// }
/// ```
pub fn ext_update<F>(updater: F) where F: Fn(ExternalBuilder) -> ExternalBuilder {
	let old_ext = get_external::<ExternalInstance>();
	let log = old_ext.log.clone();
	let calls = old_ext.calls.clone();
	let builder = ExternalBuilder::from(old_ext);
	let mut ext = updater(builder).build();
	ext.log = log;
	ext.calls = calls;
	set_external(Box::new(ext));
}

#[doc(hidden)]
/// Returns current ExternalInstance
pub fn ext_get() -> ExternalInstance {
	get_external::<ExternalInstance>()
}

/// Test with provided externals manager (`impl ::pwasm_test::External`)
#[macro_export]
macro_rules! test_with_external {
	(
		$external_instance:expr, $($test_name:ident $test_body:block)*
	) => {
		$(#[test]
		fn $test_name() {
			$crate::set_external(Box::new($external_instance));
			$test_body
		})*
	}
}
