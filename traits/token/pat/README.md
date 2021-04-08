# PAT Interface

## Usage
### Cargo.toml Setting
```
[dependencies]
pat = { package = "pat", version = "0.1.0", git = "https://github.com/patractlabs/metis.git", default-features = false, features = ["ink-as-dependency"] }

[features]
default = ["std"]
std = [
    "pat/std",
]
```
### Example Contract
```rust
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod delegate {
    use pat::{
        Pat,
        StandardToken,
    };
    use ink_env::call::FromAccountId;

    #[ink(storage)]
    pub struct Delegate {
        token: StandardToken,
    }

    impl Delegate {
        #[ink(constructor)]
        pub fn new(contract_account: AccountId) -> Self {
            let token: StandardToken = FromAccountId::from_account_id(contract_account);
            Self { token }
        }

        #[ink(message)]
        pub fn call(&self, owner: AccountId) -> Balance {
            self.token.balance_of(owner)
        }
    }
}
```
