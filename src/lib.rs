extern crate pwasm_std;
extern crate pwasm_ethereum;
extern crate bigint;
extern crate pwasm_abi;

mod external;
mod externs;
mod builder;

pub use external::{Endpoint, External, ExternalInstance, Error};
pub use builder::{ExternalBuilder};
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
