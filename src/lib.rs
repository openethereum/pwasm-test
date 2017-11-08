extern crate pwasm_std;
extern crate pwasm_abi;

pub mod external;
pub mod fake_runtime;
mod externs;

pub use external::{External, Error};
pub use externs::*;

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
