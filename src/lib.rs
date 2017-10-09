extern crate parity_hash;
extern crate bigint;

pub mod external;
mod externs;

pub use external::{External, Error};
pub use externs::*;

#[macro_export]
macro_rules! test_with_external {
    (
       $struc:ident: $imp:item $($test_name:ident $test_body:block)*
    ) => {
        struct $struc;
        $imp

        $(#[test]
        fn $test_name() {
            $crate::set_external(Box::new($struc));
            $test_body
        })*
    }
}
