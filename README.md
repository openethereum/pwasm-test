[![Build Status](https://travis-ci.org/paritytech/pwasm-test.svg?branch=master)](https://travis-ci.org/fckt/pwasm-test)

*pwasm-test* is a set of tools to make it easy to test internal logic of contracts written using [pwasm-std](https://github.com/paritytech/pwasm-std) and [pwasm-ethereum](https://github.com/paritytech/pwasm-ethereum).

## Usage

```rust
#[macro_use]
extern crate pwasm_test;
extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_std::hash::Address;
use pwasm_ethereum::ext;
use pwasm_test::{ExternalBuilder, ExternalInstance, get_external};

/// An example of how to use get_external to access "calls" to some contract
test_with_external!(
	ExternalBuilder::new().build(),
	has_called {
		// Somewhere inside of the contract:
		let mut result = [0u8; 1];
		let input = [2u8; 32];
		ext::call(&Address::new(), 10000.into(), &input, &mut result).unwrap();

		let external = get_external::<ExternalInstance>();
		let calls = external.calls();
		assert_eq!(calls.len(), 1);

		let call = &calls[0];
		assert_eq!(call.value, 10000.into());
		assert_eq!(call.address, Address::new());
		assert!(&input == call.input.as_ref());
	}
);
```

## Run

`cargo test --features=pwasm-ethereum/std`

# License

`parity-test` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0), at your choice.

See LICENSE-APACHE, and LICENSE-MIT for details.
