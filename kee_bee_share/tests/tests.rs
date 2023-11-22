use gstd::{Encode};
use gtest::{Program, System};
use kee_bee_io::*;
const USERS: &[u64] = &[3, 4, 5];

fn init_with_mint(sys: &System) {
    sys.init_logger();
    // load the contract.
    let ft = Program::current_opt(sys);

    let res = ft.send(
        USERS[0],
        InitConfig {
            protocol_fee_destination: USERS[2].into(),
            protocol_fee_percent: 50000000000000000,
            subject_fee_percent: 50000000000000000,
            max_fee_percent: 100000000000000000,
            max_amount: 1,
        },
    );

    assert!(!res.main_failed());

}

#[test]
fn buy_share() {
    let sys = System::new();
    init_with_mint(&sys);
    let ft = sys.get_program(1);
    let state:StateReply = ft.read_state(StateQuery::FullState).expect("read fullstate error");
    if let StateReply::FullState(io_kee_bee_share) = state{
        assert!(io_kee_bee_share.protocol_fee_percent==50000000000000000,"protocolFeePercent test fail");
        assert!(io_kee_bee_share.subject_fee_percent==50000000000000000,"subject_fee_percent test fail");
    }
    let buy_price:StateReply = ft.read_state(StateQuery::BuyPrice { shares_subject: USERS[1].into(), amount: 1 }).expect("read buy price error!");
    if let StateReply::Price(price) = buy_price{
        assert!(price == 0,"buy price error!");
    }

    // buy first share by other pepole failed.
    let buy_share_res = ft.send(USERS[1], KBAction::BuyShare {
        shares_subject: USERS[2].into(),
        amount: 1,
    });
    assert!(buy_share_res.main_failed());
    println!("buy_share_res is:{buy_share_res:?}");


    // buy first share by self success.
    let buy_share_res = ft.send(USERS[1], KBAction::BuyShare {
        shares_subject: USERS[1].into(),
        amount: 1,
    });
    assert!(!buy_share_res.main_failed());
    println!("buy_share_res is:{buy_share_res:?}");


    assert!(buy_share_res.contains(&(
        USERS[1],
        KBEvent::Trade {
            trader: USERS[1].into(),
            subject: USERS[2].into(),
            is_buy: true,
            share_amount: 1,
            eth_amount: 0,
            protocol_eth_amount: 0,
            subject_eth_amount: 0,
            supply: 1
        }.encode()
    )));
}

// #[test]
// fn burn() {
//     let sys = System::new();
//     init_with_mint(&sys);
//     let ft = sys.get_program(1);
//     let res = ft.send(USERS[0], FTAction::Burn(1000));
//     assert!(res.contains(&(
//         USERS[0],
//         FTEvent::Transfer {
//             from: USERS[0].into(),
//             to: 0.into(),
//             amount: 1000,
//         }
//         .encode()
//     )));
//     let res = ft.send(USERS[0], FTAction::BalanceOf(USERS[0].into()));
//     assert!(res.contains(&(USERS[0], FTEvent::Balance(999000).encode())));
// }

// #[test]
// fn burn_failures() {
//     let sys = System::new();
//     sys.init_logger();
//     init_with_mint(&sys);
//     let ft = sys.get_program(1);
//     // must fail since the amount > the user balance
//     let res = ft.send(USERS[0], FTAction::Burn(1000001));
//     assert!(res.main_failed());
// }

// #[test]
// fn transfer() {
//     let sys = System::new();
//     init_with_mint(&sys);
//     let ft = sys.get_program(1);
//     let res = ft.send(
//         USERS[0],
//         FTAction::Transfer {
//             from: USERS[0].into(),
//             to: USERS[1].into(),
//             amount: 500,
//         },
//     );

//     assert!(res.contains(&(
//         USERS[0],
//         FTEvent::Transfer {
//             from: USERS[0].into(),
//             to: USERS[1].into(),
//             amount: 500,
//         }
//         .encode()
//     )));

//     // check that the balance of `USER[0]` decreased and the balance of `USER[1]` increased
//     let res = ft.send(USERS[0], FTAction::BalanceOf(USERS[0].into()));
//     assert!(res.contains(&(USERS[0], FTEvent::Balance(999500).encode())));
//     let res = ft.send(USERS[0], FTAction::BalanceOf(USERS[1].into()));
//     assert!(res.contains(&(USERS[0], FTEvent::Balance(500).encode())));
// }

// #[test]
// fn transfer_failures() {
//     let sys = System::new();
//     init_with_mint(&sys);
//     let ft = sys.get_program(1);
//     //must fail since the amount > balance
//     let res = ft.send(
//         USERS[0],
//         FTAction::Transfer {
//             from: USERS[0].into(),
//             to: USERS[1].into(),
//             amount: 2000000,
//         },
//     );
//     assert!(res.main_failed());

//     //must fail transfer to zero address
//     let res = ft.send(
//         USERS[2],
//         FTAction::Transfer {
//             from: USERS[0].into(),
//             to: 0.into(),
//             amount: 100,
//         },
//     );
//     assert!(res.main_failed());
// }

// #[test]
// fn approve_and_transfer() {
//     let sys = System::new();
//     init_with_mint(&sys);
//     let ft = sys.get_program(1);

//     let res = ft.send(
//         USERS[0],
//         FTAction::Approve {
//             to: USERS[1].into(),
//             amount: 500,
//         },
//     );
//     assert!(res.contains(&(
//         USERS[0],
//         FTEvent::Approve {
//             from: USERS[0].into(),
//             to: USERS[1].into(),
//             amount: 500,
//         }
//         .encode()
//     )));

//     let res = ft.send(
//         USERS[1],
//         FTAction::Transfer {
//             from: USERS[0].into(),
//             to: USERS[2].into(),
//             amount: 200,
//         },
//     );
//     assert!(res.contains(&(
//         USERS[1],
//         FTEvent::Transfer {
//             from: USERS[0].into(),
//             to: USERS[2].into(),
//             amount: 200,
//         }
//         .encode()
//     )));

//     // check that the balance of `USER[0]` decreased and the balance of `USER[1]` increased
//     let res = ft.send(USERS[0], FTAction::BalanceOf(USERS[0].into()));
//     assert!(res.contains(&(USERS[0], FTEvent::Balance(999800).encode())));
//     let res = ft.send(USERS[0], FTAction::BalanceOf(USERS[2].into()));
//     assert!(res.contains(&(USERS[0], FTEvent::Balance(200).encode())));

//     // must fail since not enough allowance
//     let res = ft.send(
//         USERS[1],
//         FTAction::Transfer {
//             from: USERS[0].into(),
//             to: USERS[2].into(),
//             amount: 800,
//         },
//     );
//     assert!(res.main_failed());
// }
