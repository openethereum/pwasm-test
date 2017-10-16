#[macro_use]
extern crate pwasm_test;
extern crate pwasm_std;

use pwasm_std::hash::H256;
use pwasm_std::storage;
use pwasm_test::External;

use std::collections::HashMap;

test_with_external!(
    DummyExternal: impl External for DummyExternal {
        fn storage(&mut self) -> HashMap<H256, [u8; 32]> {
            let mut storage = HashMap::new();
            storage.insert(H256::new(), [250; 32]);
            storage
        }
    }
    check_balance {
        assert_eq!([250; 32], storage::read(&H256::new()).unwrap());
    }
);
