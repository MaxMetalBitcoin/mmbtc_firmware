#![no_std]

use heapless::Vec;

use crate::menu::confirm_mainnet_chosen_menu_helpers;
use crate::menu::menu_screen_state::MenuScreenTypesState;
use crate::menu::menu_screen_state::MenuTypes;
use crate::networks;

pub fn init_choose_network_menu() -> MenuScreenTypesState {
    let mut choices: Vec<&'static str, 20> = Vec::new();

    choices.push(" Main").unwrap();
    choices.push(" Testnet").unwrap();
    choices.push(" Signet").unwrap();

    MenuScreenTypesState {
        prompt: "Choose your network:",
        choices: choices,
        hover_index: 0,
        menu_type: MenuTypes::ChooseNetworkMenuType,
    }
}

pub fn get_network_from_choice(
    menu_screen_types_state: &MenuScreenTypesState,
) -> networks::Networks {
    match menu_screen_types_state.hover_index {
        0 => networks::Networks::Mainnet,
        1 => networks::Networks::Testnet,
        2 => networks::Networks::Signet,
        _ => panic!("Invalid state - only 3 network choices"),
    }
}

pub fn get_next_menu_on_choose_network_menu(
    menu_screen_types_state: &MenuScreenTypesState,
) -> MenuScreenTypesState {
    match menu_screen_types_state.hover_index {
        0 => confirm_mainnet_chosen_menu_helpers::init_confirm_mainnet_chosen_menu(),
        1 => confirm_mainnet_chosen_menu_helpers::init_confirm_mainnet_chosen_menu(),
        2 => confirm_mainnet_chosen_menu_helpers::init_confirm_mainnet_chosen_menu(),
        _ => panic!("Invalid state - only 3 network choices"),
    }
}
