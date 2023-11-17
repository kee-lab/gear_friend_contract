use gmeta::metawasm;
use gstd::prelude::*;

#[metawasm]
pub mod metafns {
    pub type State = Option<KeeBeeShare>;

    pub fn wallet_by_id(state: State, id: Id) -> Option<Wallet> {
        state.into_iter().find(|w| w.id == id)
    }

    pub fn wallet_by_person(state: State, person: String) -> Option<Wallet> {
        state.into_iter().find(|w| w.person == person)
    }
}