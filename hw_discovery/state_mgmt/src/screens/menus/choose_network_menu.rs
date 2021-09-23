#![no_std]

extern crate alloc;

use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use core::{fmt::Error, ops::Deref};
use embedded_graphics::{
    drawable::Drawable,
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    text_style,
};
use heapless::Vec;

use crate::networks;
use crate::{display_type, networks::Networks};
use crate::{menu::choose_network_menu_helpers, mm_state::MMState};
use crate::{menu::confirm_mainnet_chosen_menu_helpers, mm_state::ScreenTypes};
use crate::{mm_state::MenuTypes, mm_state_action};

use super::confirm_mainnet_chosen_menu;

pub fn initialize_menu(mut state: &mut MMState) -> bool {
    let mut menu_choices: Vec<Box<String>, 20> = Vec::new();

    menu_choices.push(Box::new(" Main".to_string())).unwrap();
    menu_choices.push(Box::new(" Testnet".to_string())).unwrap();
    menu_choices.push(Box::new(" Signet".to_string())).unwrap();

    let raw = "L1HKVVLHXiUhecWnwFYF6L3shkf1E12HUmuZTESvBXUdx3yqVP1D";
    let pk = bitcoin::PrivateKey::from_wif(raw).unwrap();

    state.menu_choices = menu_choices;
    state.current_screen = ScreenTypes::MenuScreenType;
    state.menu_type = MenuTypes::ChooseNetworkMenuType;
    state.menu_prompt = Box::new("Choose you network:".to_string());
    state.private_key = pk;

    true
}

pub fn menu_item_selected(mut state: &mut MMState) -> bool {
    if state
        .menu_choices
        .get(state.menu_hover_index as usize)
        .unwrap()
        .deref()
        .deref()
        .eq(" Main")
    {
        confirm_mainnet_chosen_menu::initialize_menu(&mut state)
    } else {
        if state
            .menu_choices
            .get(state.menu_hover_index as usize)
            .unwrap()
            .deref()
            .deref()
            .eq(" Testnet")
        {
            state.network = Networks::Testnet;
            true
        } else {
            state.network = Networks::Signet;
            true
        }
    }
}
