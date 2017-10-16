#[macro_use]
extern crate pwasm_test;
extern crate pwasm_std;

use pwasm_std::hash::Address;
use pwasm_std::bigint::U256;
use pwasm_std::ext;
use pwasm_test::External;

use std::collections::HashMap;

test_with_external!(
    DummyExternal: impl External for DummyExternal {
        fn balances(&mut self) -> HashMap<Address, U256> {
            let addr = Address::from([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]);
            let mut balances: HashMap<Address, U256> = HashMap::new();
            balances.insert(addr, U256::from(200000));
            balances
        }
    }
    check_balance {
        assert_eq!(U256::from(200000), ext::balance(&Address::from([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20])));
        }
);
