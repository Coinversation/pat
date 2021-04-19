#![cfg_attr(not(feature = "std"), no_std)]

pub use self::pat::PatStub;
use ink_lang as ink;

#[ink::contract]
mod pat {
    use ink_prelude::string::String;

    /// The PAT error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if not enough balance to fulfill a request is available.
        InsufficientBalance,
        /// Returned if not enough allowance to fulfill a request is available.
        InsufficientAllowance,
    }

    /// The PAT result type.
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(storage)]
    pub struct PatStub {}

    impl PatStub {
        /// Creates a new PAT contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(
            _initial_supply: Balance,
            _name: Option<String>,
            _symbol: Option<String>,
            _decimals: Option<u8>,
        ) -> Self {
            unimplemented!()
        }

        /// Returns the token name.
        #[ink(message, selector = "0xd3dc9f8c")]
        pub fn token_name(&self) -> Option<String> {
            unimplemented!()
        }

        /// Returns the token symbol.
        #[ink(message, selector = "0xe4ab4943")]
        pub fn token_symbol(&self) -> Option<String> {
            unimplemented!()
        }

        /// Returns the token decimals.
        #[ink(message, selector = "0xc4b508e2")]
        pub fn token_decimals(&self) -> Option<u8> {
            unimplemented!()
        }

        /// Returns the total token supply.
        #[ink(message, selector = "0x557c8bd0")]
        pub fn total_supply(&self) -> Balance {
            unimplemented!()
        }

        /// Returns the account balance for the specified `owner`.
        #[ink(message, selector = "0xb13e6c24")]
        pub fn balance_of(&self, _owner: AccountId) -> Balance {
            unimplemented!()
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        #[ink(message, selector = "0xa6b726c7")]
        pub fn transfer(&mut self, _to: AccountId, _value: Balance) -> Result<()> {
            unimplemented!()
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        #[ink(message, selector = "0x984f2ea5")]
        pub fn allowance(&self, _owner: AccountId, _spender: AccountId) -> Balance {
            unimplemented!()
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        #[ink(message, selector = "0xeb7943df")]
        pub fn transfer_from(
            &mut self,
            _from: AccountId,
            _to: AccountId,
            _value: Balance,
        ) -> Result<()> {
            unimplemented!()
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        #[ink(message, selector = "0x702770d4")]
        pub fn approve(&mut self, _spender: AccountId, _value: Balance) -> Result<()> {
            unimplemented!()
        }
    }
}
