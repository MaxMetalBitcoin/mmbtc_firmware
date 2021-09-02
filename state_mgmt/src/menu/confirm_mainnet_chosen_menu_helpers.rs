#![no_std]

use heapless::Vec;

use crate::menu::choose_network_menu_helpers;
use crate::menu::menu_screen_state::MenuScreenTypesState;
use crate::menu::menu_screen_state::MenuTypes;
use crate::networks;

pub fn init_confirm_mainnet_chosen_menu() -> MenuScreenTypesState {
    let mut choices: Vec<&'static str, 20> = Vec::new();

    choices.push(" No").unwrap();
    choices.push(" Yes").unwrap();

    MenuScreenTypesState {
        prompt: "Are you sure? This is REAL money:",
        choices: choices,
        hover_index: 0,
        menu_type: MenuTypes::ConfirmMainnetMenuType,
    }
}

pub fn get_next_menu_on_confirm_mainnet_menu(
    menu_screen_types_state: &MenuScreenTypesState,
) -> MenuScreenTypesState {
    match menu_screen_types_state.hover_index {
        0 => choose_network_menu_helpers::init_choose_network_menu(),
        1 => init_confirm_mainnet_chosen_menu(),
        _ => panic!("Invalid state - only 2 choices, yes or no"),
    }
}
