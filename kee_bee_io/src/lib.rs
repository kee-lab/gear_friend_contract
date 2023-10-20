#![no_std]

use gmeta::{Metadata, InOut, In, Out};
use gstd::{ActorId, Decode, Encode, TypeInfo, String, Vec};

pub struct KeeBeeMetadata;

impl Metadata for KeeBeeMetadata {
    type Init = In<InitConfig>;
    type Handle = InOut<KBAction, FBSEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<IoKeeBeeShare>;
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitConfig {
    pub protocol_fee_destination: ActorId,
    pub protocol_fee_percent:u128,
    pub subject_fee_percent:u128,
    pub max_fee_percent:u128,
    pub max_amount:u8,
}


#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum KBAction {
    BuyShare {
        shares_subject: ActorId,
        amount: u128,
    },
    SellShare {
        shares_subject: ActorId,
        amount: u128,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FBSEvent {
    Trade {
        trader: ActorId,
        subject: ActorId,
        is_buy: bool,
        share_amount: u128,
        eth_amount: u128,
        protocol_eth_amount: u128,
        subject_eth_amount: u128,
        supply: u128,
    },
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoKeeBeeShare {
    pub shares_balance: Vec<(ActorId, Vec<(ActorId, u128)>)>,
    pub share_supply: Vec<(ActorId, u128)>,
    pub manager: Vec<(ActorId, bool)>,
    pub protocol_fee_destination: ActorId,
    pub protocol_fee_percent: u128,
    pub subject_fee_percent: u128,
    pub max_fee_percent: u128,
    pub max_amount: u8,
}