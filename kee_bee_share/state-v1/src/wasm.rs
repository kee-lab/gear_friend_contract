use gmeta::metawasm;
use gstd::prelude::*;
use kee_bee_io::KeeBeeShare;

const ETH1: u128 = 10u128.pow(18);

#[metawasm]
pub mod metafns {
    pub type State = Option<KeeBeeShare>;

    pub fn getPrice(state:State,supply:u128,amount:u128)->u128{
        // uint256 sum1 = supply == 0 ? 0 : (supply - 1 )* (supply) * (2 * (supply - 1) + 1) / 6;
        let sum1 = if supply ==0 {
            0
        }else{
            (supply - 1 )* (supply) * (2 * (supply - 1) + 1) / 6
        };
        let sum2 = if supply == 0 && amount == 1 {
            0
        }else{
            (supply - 1 + amount) * (supply + amount) * (2 * (supply - 1 + amount) + 1) / 6
        };
        let summation = sum2 - sum1;
        summation * 10u128.pow(18) / 16000
    }

    // pub fn wallet_by_id(state: State, id: Id) -> Option<Wallet> {
    //     state.into_iter().find(|w| w.id == id)
    // }

    // pub fn wallet_by_person(state: State, person: String) -> Option<Wallet> {
    //     state.into_iter().find(|w| w.person == person)
    // }
}