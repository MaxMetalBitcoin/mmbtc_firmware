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

    menu_choices.push(Box::new("QR Code".to_string())).unwrap();
    menu_choices.push(Box::new("SD Card".to_string())).unwrap();

    state.current_screen = ScreenTypes::MenuScreenType;
    state.menu_prompt = Box::new("Load Transaction From:".to_string());
    state.menu_choices = menu_choices;
    state.menu_hover_index = 0;
    state.menu_type = MenuTypes::WhereToLoadTransactionMenuType;

    true
}

pub fn menu_item_selected(mut state: &mut MMState) -> bool {
    choose_network_menu::initialize_menu(&mut state)
}
