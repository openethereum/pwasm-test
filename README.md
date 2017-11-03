[![Build Status](https://travis-ci.org/paritytech/pwasm-test.svg?branch=master)](https://travis-ci.org/fckt/pwasm-test)

*pwasm-test* is a set of tools to make it easy to test internal logic of contracts written using [pwasm-std](https://github.com/paritytech/pwasm-std).

## Usage

```rust

#[macro_use]
extern crate pwasm_std;
#[macro_use]
extern crate pwasm_test;

use pwasm_std::{storage, ext};
use pwasm_std::hash::H256;
use pwasm_std::bigint::U256;
use pwasm_test::{Error, External};

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyExternal;

    impl External for DummyExternal {
        fn storage_read(&mut self, key: &H256) -> Result<[u8; 32], Error> {
            Ok([1u8; 32])
        }

        fn value(&mut self) -> U256 {
            500.into()
        }
    }

    test_with_external!(
        DummyExternal,
        simple_test1 {
            let val = storage::read(&H256::from("68371d7e884c168ae2022c82bd837d51837718a7f7dfb7aa3f753074a35e1d87"));
            assert_eq!(val.unwrap(), [1u8; 32]);
        }
        simple_test2 {
            assert_eq!(ext::value(), 500.into());
        }
    );
}
```

## Run

`cargo test --features=pwasm-std/std`

# License

`parity-wasm` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0), at your choice.

See LICENSE-APACHE, and LICENSE-MIT for details.
