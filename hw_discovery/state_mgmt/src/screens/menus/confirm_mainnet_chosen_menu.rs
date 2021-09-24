#![no_std]

extern crate alloc;

use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use heapless::Vec;

use crate::mm_state::MMState;
use crate::mm_state::ScreenTypes;
use crate::screens::menus::test_signing_menu;

use super::MenuTypes;
use super::*;

pub fn initialize_menu(mut state: &mut MMState) -> bool {
    let mut menu_choices: Vec<Box<String>, 20> = Vec::new();

    menu_choices.push(Box::new(" No".to_string())).unwrap();
    menu_choices.push(Box::new(" Yes".to_string())).unwrap();

    state.current_screen = ScreenTypes::MenuScreenType;
    state.menu_prompt =
        Box::new("Are you sure that you want to use mainnet (real money)?:".to_string());
    state.menu_choices = menu_choices;
    state.menu_hover_index = 0;
    state.menu_type = MenuTypes::ConfirmMainnetMenuType;

    true
}

pub fn menu_item_selected(mut state: &mut MMState) -> bool {
    match state.menu_hover_index {
        0 => choose_network_menu::initialize_menu(&mut state),
        1 => test_signing_menu::initialize_menu(&mut state),
        _ => false,
    }
}
