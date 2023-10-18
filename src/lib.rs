#![no_std]

use gstd::{collections::HashMap, exec, msg, prelude::*, ActorId};

static mut KEE_BEE_SHARE: Option<KeeBeeShare> = None;
static mut PROTOCOL_FEE_PERCENT: u128 = 50000000000000000;
static mut SUBJECT_FEE_PERCENT: u128 = 50000000000000000;
static mut MAX_FEE_PERCENT: u128 = 100000000000000000;
const ETH1: u128 = 10 ^ 18;

#[derive(Debug, Default)]
pub struct KeeBeeShare {
    // address public protocolFeeDestination;
    pub protocol_fee_destination: ActorId,
    // mapping(address => mapping(address => uint256)) public sharesBalance;
    pub shares_balance: HashMap<ActorId, HashMap<ActorId, u128>>,

    // SharesSubject => Supply
    // mapping(address => uint256) public sharesSupply;
    pub share_supply: HashMap<ActorId, u128>,
    pub manager: HashMap<ActorId, bool>,
    pub max_amount:u8,
}

#[no_mangle]
extern fn init() {
    let protocol_fee_destination: ActorId =
        msg::load().expect("Unable to decode protocoal fee destination");
    let mut kee_bee_share = KeeBeeShare {
        protocol_fee_destination: protocol_fee_destination,
        max_amount:1,
        ..Default::default()
    };
    kee_bee_share.manager.insert(msg::source(), true);
    unsafe {
        KEE_BEE_SHARE = Some(kee_bee_share);
    }
    // isManager[msg.sender] = true;
    // protocolFeeDestination = _protocolFeeDestination;
}

// event Trade(address trader, address subject, bool isBuy, uint256 shareAmount, uint256 ethAmount, uint256 protocolEthAmount, uint256 subjectEthAmount, uint256 supply);


impl KeeBeeShare {
    fn getPrice(&self,supply: u128, amount: u128) -> u128 {
        assert!(amount <= self.max_amount.into(), "amount too high");
        let sum1 = if supply == 0 {
            0
        } else {
            (supply - 1) * (supply) * (2 * (supply - 1) + 1) / 6
        };
        let sum2 = if supply == 0 && amount == 1 {
            0
        } else {
            (supply - 1 + amount) * (supply + amount) * (2 * (supply - 1 + amount) + 1) / 6
        };
        let summation = sum2 - sum1;
        return summation * ETH1 / 16000u128;
    }
}
