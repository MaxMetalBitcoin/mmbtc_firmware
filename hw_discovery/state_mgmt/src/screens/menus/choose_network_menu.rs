#![no_std]

extern crate alloc;

use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use heapless::Vec;

use crate::mm_state::MMState;
use crate::mm_state::ScreenTypes;
use crate::networks::Networks;

use super::MenuTypes;
use super::*;

pub fn initialize_menu(mut state: &mut MMState) -> bool {
    let mut menu_choices: Vec<Box<String>, 20> = Vec::new();

    menu_choices.push(Box::new(" Main".to_string())).unwrap();
    menu_choices.push(Box::new(" Testnet".to_string())).unwrap();
    menu_choices.push(Box::new(" Signet".to_string())).unwrap();

    let raw = "L1HKVVLHXiUhecWnwFYF6L3shkf1E12HUmuZTESvBXUdx3yqVP1D";
    let pk = bitcoin::PrivateKey::from_wif(raw).unwrap();
    state.private_key = pk;

    state.current_screen = ScreenTypes::MenuScreenType;
    state.menu_prompt = Box::new("Choose you network:".to_string());
    state.menu_choices = menu_choices;
    state.menu_hover_index = 0;
    state.menu_type = MenuTypes::ChooseNetworkMenuType;

    true
}

pub fn menu_item_selected(mut state: &mut MMState) -> bool {
    match state.menu_hover_index {
        0 => confirm_mainnet_chosen_menu::initialize_menu(&mut state),
        1 => {
            state.network = Networks::Testnet;
            main_menu::initialize_menu(&mut state)
        }
        2 => {
            state.network = Networks::Signet;
            main_menu::initialize_menu(&mut state)
        }
        _ => false,
    }
}
