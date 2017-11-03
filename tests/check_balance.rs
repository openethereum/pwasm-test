#[macro_use]
extern crate pwasm_test;
extern crate pwasm_std;

use pwasm_std::hash::Address;
use pwasm_std::bigint::U256;
use pwasm_std::ext;
use pwasm_test::External;

use std::collections::HashMap;

#[derive(Default)]
struct DummyExternal {
    balances: HashMap<Address, U256>,
}

impl DummyExternal {
    fn with_balances() -> Self {
        let mut balances = HashMap::new();
        balances.insert(
            Address::from([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]),
            U256::from(200000)
        );

        DummyExternal { balances: balances }
    }
}

impl External for DummyExternal {
    fn balance(&mut self, address: &Address) -> U256 {
        self.balances[address]
    }
}

test_with_external!(
    DummyExternal::with_balances(),
    check_balance {
        assert_eq!(
            U256::from(200000),
            ext::balance(&Address::from([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]))
        );
    }
);
