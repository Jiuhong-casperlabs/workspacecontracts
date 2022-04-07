#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, vec};

use casper_contract::contract_api::{runtime, storage};
use casper_types::{Key, URef, U512};

#[no_mangle]
pub extern "C" fn call() {
    let player1uref = storage::new_dictionary("player1").unwrap();
    // [totalgames, wins, losses, stalemates, +increment game_history]
    storage::dictionary_put(
        player1uref,
        "totalgames",
        U512::from_dec_str("111111111111111111111111111").unwrap(),
    );

    storage::dictionary_put(player1uref, "wins", U512::from_dec_str("22").unwrap());

    storage::dictionary_put(player1uref, "losses", U512::from_dec_str("33").unwrap());

    storage::dictionary_put(player1uref, "stalemates", U512::from_dec_str("44").unwrap());

    storage::dictionary_put(
        player1uref,
        "game_history",
        vec![String::from("hello"), String::from("world")],
    );

    let playersuref = storage::new_dictionary("playersuref").unwrap();

    storage::dictionary_put(playersuref, "player1", player1uref);

    let player1uref: URef = storage::dictionary_get(playersuref, "player1")
        .unwrap()
        .unwrap();

    let totalgames: U512 = storage::dictionary_get(player1uref, "totalgames")
        .unwrap()
        .unwrap();

    let game_history: Vec<String> = storage::dictionary_get(player1uref, "game_history")
        .unwrap()
        .unwrap();

    runtime::put_key("totalgames", storage::new_uref(totalgames).into());
    runtime::put_key("game_history", storage::new_uref(game_history).into());
}
