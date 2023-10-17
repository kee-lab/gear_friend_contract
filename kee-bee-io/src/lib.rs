use gmeta::{Metadata, InOut, In, Out};
use gstd::{ActorId, Decode, Encode, TypeInfo};

pub struct KeeBeeMetadata;

impl Metadata for KeeBeeMetadata {
    type Init = In<ActorId>;
    type Handle = InOut<FTAction, FTEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<IoFungibleToken>;
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
pub enum FTEvent {
    Transfer {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    TotalSupply(u128),
    Balance(u128),
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