#![no_std]

use gmeta::{Metadata, InOut, In, Out};
use gstd::{ActorId, Decode, Encode, TypeInfo, String, Vec};

pub struct KeeBeeMetadata;

impl Metadata for KeeBeeMetadata {
    type Init = In<InitConfig>;
    type Handle = InOut<FTAction, FBSEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<IoFungibleToken>;
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
pub enum FTAction {
    Mint(u128),
    Burn(u128),
    Transfer {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        to: ActorId,
        amount: u128,
    },
    TotalSupply,
    BalanceOf(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FBSEvent {
    Trade {
        trader: ActorId,
        subject: ActorId,
        isBuy: bool,
        shareAmount: u128,
        ethAmount: u128,
        protocolEthAmount: u128,
        subjectEthAmount: u128,
        supply: u128,
    },
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoFungibleToken {
    pub name: String,
    pub symbol: String,
    pub total_supply: u128,
    pub balances: Vec<(ActorId, u128)>,
    pub allowances: Vec<(ActorId, Vec<(ActorId, u128)>)>,
    pub decimals: u8,
}