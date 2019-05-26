// For the original code, visit:
// https://github.com/paritytech/ink/blob/master/examples/lang/erc20/src/lib.rs

#![cfg_attr(not(any(test, feature = "test-env")), no_std)]

use ink_core::{
    env::{
        self,
        AccountId,
        Balance,
    },
    memory::format,
    storage,
};
use ink_lang::contract;

contract! {
    // Event deposited when a token transfer occurs
    event Transfer {
        from: Option<AccountId>,
        to: Option<AccountId>,
        value: Balance,
    }

    // Event deposited when an approval occurs
    event Approval {
        owner: AccountId,
        spender: AccountId,
        value: Balance,
    }

    /// The storage items for a typical ERC20 token implementation.
    struct Trackvestor {
        /// The total supply.
        total_supply: storage::Value<Balance>,
        /// The balance of each user.
        balances: storage::HashMap<AccountId, Balance>,
        /// Balances that are spendable by non-owners: (owner, spender) -> allowed
        allowances: storage::HashMap<(AccountId, AccountId), Balance>,
    }

    impl Deploy for Trackvestor {
        fn deploy(&mut self, init_value: Balance) {
            self.total_supply.set(init_value);
            self.balances.insert(env.caller(), init_value);
            env.emit(Transfer {
                from: None,
                to: Some(env.caller()),
                value: init_value
            });
        }
    }

    impl Trackvestor {
        /// Returns the total number of tokens in existence.
        pub(external) fn total_supply(&self) -> Balance {
            let total_supply = *self.total_supply;
            env.println(&format!("Trackvestor::total_supply = {:?}", total_supply));
            total_supply
        }

        /// Returns the balance of the given AccountId.
        pub(external) fn balance_of(&self, owner: AccountId) -> Balance {
            let balance = self.balance_of_or_zero(&owner);
            env.println(&format!("Trackvestor::balance_of(owner = {:?}) = {:?}", owner, balance));
            balance
        }

        /// Returns the amount of tokens that an owner allowed to a spender.
        pub(external) fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            let allowance = self.allowance_or_zero(&owner, &spender);
            env::println(&format!(
                "Trackvestor::allowance(owner = {:?}, spender = {:?}) = {:?}",
                owner, spender, allowance
            ));
            allowance
        }

        /// Transfers token from the sender to the `to` AccountId.
        pub(external) fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            self.transfer_impl(env, env.caller(), to, value)
        }

        /// Approve the passed AccountId to spend the specified amount of tokens
        /// on the behalf of the message's sender.
        pub(external) fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
            let owner = env.caller();
            self.allowances.insert((owner, spender), value);
            env.emit(Approval {
                owner: owner,
                spender: spender,
                value: value
            });
            true
        }

        /// Transfer tokens from one AccountId to another.
        pub(external) fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let allowance = self.allowance_or_zero(&from, &env.caller());
            if allowance < value {
                return false
            }
            self.allowances.insert((from, env.caller()), allowance - value);
            self.transfer_impl(env, from, to, value)
        }
    }

    impl Trackvestor {
        /// Returns the balance of the AccountId or 0 if there is no balance.
        fn balance_of_or_zero(&self, of: &AccountId) -> Balance {
            *self.balances.get(of).unwrap_or(&0)
        }

        /// Returns the allowance or 0 of there is no allowance.
        fn allowance_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            *self.allowances.get(&(*owner, *spender)).unwrap_or(&0)
        }

        /// Transfers token from a specified AccountId to another AccountId.
        fn transfer_impl(&mut self, env: &mut ink_model::EnvHandler, from: AccountId, to: AccountId, value: Balance) -> bool {
            let balance_from = self.balance_of_or_zero(&from);
            let balance_to = self.balance_of_or_zero(&to);
            if balance_from < value {
                return false
            }
            self.balances.insert(from, balance_from - value);
            self.balances.insert(to, balance_to + value);
            env.emit(Transfer {
                from: Some(from),
                to: Some(to),
                value: value
            });
            true
        }
    }
}