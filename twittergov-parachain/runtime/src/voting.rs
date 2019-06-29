#![cfg_attr(not(feature = "std"), no_std)]


use support::traits::{Currency, WithdrawReason, ExistenceRequirement};
use runtime_primitives::traits::{ Zero, Saturating };
use support::{decl_module, decl_storage, decl_event, dispatch::Result};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use rstd::vec::Vec;


pub trait Trait: system::Trait + balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}


#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Proposal<Balance, AccountId> {
    votes: Vec<(AccountId, u64)>,
    detail: Vec<u8>,
    choices: Vec<(u64, Vec<u8>)>,
    proposer: AccountId,
    pot: Balance,
    status: StatusType
}


#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq)]
pub enum StatusType {
    Initiated,
    Voting,
    Enforcing,
    Expired,
    Removed,
}


impl Default for StatusType {
    fn default() -> Self { StatusType::Initiated }
}


fn to_u8a(from: &str) -> Vec<u8> {
    return String::from(from).into_bytes();
}


decl_storage! {
    trait Store for Module<T: Trait> as DaoModule {
        // Declare storage and getter functions here
        ProposalRecord: map T::Hash => Proposal<T::Balance, T::AccountId>
    }
}


decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn initiate(proposer: T::AccountId, ) {
            nw
        }
    }
}


decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		SomethingStored(u32, AccountId),
	}
);


#[cfg(test)]
    mod tests {
    use super::*;

    #[test]
    fn u8a() {
        let sparkle_heart = vec![240, 159, 146, 150];
        let bytes = to_u8a("💖");
        print!("{:?}", bytes);

        assert_eq!(bytes, [240, 159, 146, 150]);
    }
}