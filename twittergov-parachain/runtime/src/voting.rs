use parity_codec::alloc::collections::HashMap;
use support::traits::{Currency, WithdrawReason, ExistenceRequirement};
use runtime_primitives::traits::{ Zero, Saturating };
use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result};
use std::option::Option;



pub trait Trait: system::Trait {

    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

pub struct Proposal {
    pub votes: HashMap<str, u8>,
    pub detail: &'static str,
    pub choices: HashMap<u8, &'static str>,
    pub proposer: <T as system::Trait>::AccountId,
    pub pot: <T as system::Trait>::Balance,
    pub status: ProposalEvent
}

enum ProposalEvent {
    Initiated,
    Voting,
    Enforcing,
    Removed,
    Completed
}


decl_storage! {
    trait Store for Module<T: Trait> as DaoStorage {
        // Declare storage and getter functions here
        ProposalRecord: HashMap<U8a, Proposal>
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here
        commit()
        remove()
        vote()
    }
}

decl_event!{
pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		ProposalInit(),
		ProposalVoting(),
		ProposalRemoved(),
		ProposalEnforced(),
        ProposalCompleted()
	}
}