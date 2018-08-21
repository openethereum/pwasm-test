[![Build Status](https://travis-ci.org/paritytech/pwasm-test.svg?branch=master)](https://travis-ci.org/paritytech/pwasm-test)

`pwasm-test` is a set of tools to make it easy to test internal logic of contracts written using [pwasm-ethereum](https://github.com/paritytech/pwasm-ethereum).

## Usage
Let's assume we have a simple TokenContract. Let's see how we use `pwasm_test` to mock `pwasm_ethereum::sender()` call:

```rust
extern crate parity_hash;
extern crate pwasm_ethereum;
extern crate uint;

use parity_hash::{H256, Address};
use pwasm_ethereum;
use uint::U256;
use pwasm_abi_derive::eth_abi;

static TOTAL_SUPPLY_KEY: H256 = H256([2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
static OWNER_KEY: H256 = H256([3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);

pub trait TokenContract {
	fn constructor(&mut self, _total_supply: U256);
	#[constant]
	fn totalSupply(&mut self) -> U256;
	#[constant]
	fn balanceOf(&mut self, _owner: Address) -> U256;
	fn transfer(&mut self, _to: Address, _amount: U256) -> bool;
	/// Event declaration
	#[event]
	fn Transfer(&mut self, indexed_from: Address, indexed_to: Address, _value: U256);
}

pub struct TokenContractInstance;

// Reads balance by address
fn read_balance_of(owner: &Address) -> U256 {
	pwasm_ethereum::read(&balance_key(owner)).into()
}

// Generates a balance key for some address.
// Used to map balances with their owners.
fn balance_key(address: &Address) -> H256 {
	let mut key = H256::from(address);
	key[0] = 1; // just a naiive "namespace";
	key
}

impl TokenContract for TokenContractInstance {
	fn constructor(&mut self, total_supply: U256) {
		let sender = pwasm_ethereum::sender();
		// Set up the total supply for the token
		pwasm_ethereum::write(&TOTAL_SUPPLY_KEY, &total_supply.into());
		// Give all tokens to the contract owner
		pwasm_ethereum::write(&balance_key(&sender), &total_supply.into());
		// Set the contract owner
		pwasm_ethereum::write(&OWNER_KEY, &H256::from(sender).into());
	}

	fn totalSupply(&mut self) -> U256 {
		pwasm_ethereum::read(&TOTAL_SUPPLY_KEY).into()
	}

	fn balanceOf(&mut self, owner: Address) -> U256 {
		read_balance_of(&owner)
	}

	fn transfer(&mut self, to: Address, amount: U256) -> bool {
		let sender = pwasm_ethereum::sender();
		let senderBalance = read_balance_of(&sender);
		let recipientBalance = read_balance_of(&to);
		if amount == 0.into() || senderBalance < amount {
			false
		} else {
			let new_sender_balance = senderBalance - amount;
			let new_recipient_balance = recipientBalance + amount;
			pwasm_ethereum::write(&balance_key(&sender), &new_sender_balance.into());
			pwasm_ethereum::write(&balance_key(&to), &new_recipient_balance.into());
			self.Transfer(sender, to, amount);
			true
		}
	}
}

#[cfg(test)]
#[macro_use]
extern crate pwasm_test;

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    extern crate std;
    use super::*;
    use pwasm_test::{ext_reset, ext_get};
    use parity_hash::Address;

    #[test]
    fn should_succeed_transfering_1000_from_owner_to_another_address() {
        let mut contract = TokenContractInstance{};
        let owner_address = Address::from("0xea674fdde714fd979de3edf0f56aa9716b898ec8");
        let sam_address = Address::from("0xdb6fd484cfa46eeeb73c71edee823e4812f9e2e1");
        // Here we're creating an External context using ExternalBuilder and set the `sender` to the `owner_address`
        // so `pwasm_ethereum::sender()` in TokenContract::constructor() will return that `owner_address`
        ext_reset(|e| e.sender(owner_address.clone()));
        let total_supply = 10000.into();
        contract.constructor(total_supply);
        assert_eq!(contract.balanceOf(owner_address), total_supply);
        assert_eq!(contract.transfer(sam_address, 1000.into()), true);
        assert_eq!(contract.balanceOf(owner_address), 9000.into());
        assert_eq!(contract.balanceOf(sam_address), 1000.into());
		// 1 log entry should be created
        assert_eq!(ext_get().logs().len(), 1);
    }
}
```
For more usage examples take a look:
* https://github.com/paritytech/pwasm-token-example/blob/master/contract/src/lib.rs
* https://github.com/paritytech/pwasm-repo-contract/blob/master/contract/src/lib.rs

## Run tests

`cargo test --all`

[Parity Wasm Tutorial](https://github.com/paritytech/pwasm-tutorial) - a full fledged tutorial on how to write contracts in Webassembly for Kovan and other Wasm-enabled networks.

# License

`parity-test` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0), at your choice.

See LICENSE-APACHE, and LICENSE-MIT for details.
