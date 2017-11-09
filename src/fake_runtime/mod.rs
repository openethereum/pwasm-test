use std::collections::HashMap;
use std::mem;

use pwasm_std::hash::{Address, H256};
use pwasm_std::bigint::U256;

use super::External;
use super::Error;

// TODO: maybe extend to be enum Contract/External
pub struct Account {
	balance: U256,
	endpoint: Box<Fn(&[u8]) -> Vec<u8>>,
	storage: HashMap<H256, [u8; 32]>
}

struct FakeRuntime {
	sender: Address,
	accounts: HashMap<Address, Account>,
	call_stack: Option<Box<Call>>
}

pub struct Call {
	address: Address,
	value: U256,
	called_by: Option<Box<Call>>
}


impl FakeRuntime {
	fn new(sender: Address, accounts: HashMap<Address, Account>) -> Self {
		FakeRuntime {
			sender: sender,
			accounts: accounts,
			call_stack: None
		}
	}
}

impl External for FakeRuntime {
	fn call(&mut self, address: &Address, val: U256, input: &[u8], result: &mut [u8]) -> Result<(), Error> {
		self.call_stack = Some(Box::new(Call{
			address: address.clone(),
			value: val,
			called_by: mem::replace(&mut self.call_stack, None)
		}));
		let res = (self.accounts.get_mut(address).ok_or(Error)?.endpoint)(input);
		result.copy_from_slice(&res);
		self.call_stack = match mem::replace(&mut self.call_stack, None) {
			Some(call) => call.called_by,
			None => None
		};
		Ok(())
	}
	fn value(&mut self) -> U256 {
		match self.call_stack {
			Some(ref stack) => stack.value.clone(),
			None => 0.into()
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use super::super::{set_external, EXTERNAL};
	use pwasm_std::hash::{Address, H256};
	use pwasm_std::ext;
	#[test]
	fn test_call() {

		let contract = Account{
			balance: 0.into(),
			endpoint: Box::new(|payload| vec![1u8]),
			storage: HashMap::new()
		};
		let mut accounts = HashMap::new();
		accounts.insert("0x5484438c9bb11deeb87de29d7bf83c5d71dfd000".into(), contract);

		let mut runtime = FakeRuntime::new("0x16a0772b17ae004e6645e0e95bf50ad69498a34e".into(), accounts);
		let mut result = [0];
		runtime.call(&"0x5484438c9bb11deeb87de29d7bf83c5d71dfd000".into(), 2.into(), &[0u8], &mut result);
		assert_eq!(result, [1u8]);
	}

	#[test]
	fn test_external() {
		let contract = Account{
			balance: 0.into(),
			endpoint: Box::new(|payload| {
				assert_eq!(ext::value(), 2.into());
				vec![1u8]
			}),
			storage: HashMap::new()
		};
		let mut accounts = HashMap::new();
		accounts.insert("0x5484438c9bb11deeb87de29d7bf83c5d71dfd000".into(), contract);

		let runtime = set_external(Box::new(FakeRuntime::new("0x16a0772b17ae004e6645e0e95bf50ad69498a34e".into(), accounts)));
		EXTERNAL.with(|r| {
			let mut result = [0];
			r.borrow_mut().call(&"0x5484438c9bb11deeb87de29d7bf83c5d71dfd000".into(), 2.into(), &[0u8], &mut result);
			assert_eq!(result, [1u8]);
		})
	}
}
