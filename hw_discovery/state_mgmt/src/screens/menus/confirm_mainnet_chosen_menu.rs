#![no_std]

extern crate alloc;

use alloc::boxed::Box;
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

use super::choose_network_menu;

pub fn initialize_menu(mut state: &mut MMState) -> bool {
    let mut menu_choices: Vec<Box<&str>, 20> = Vec::new();

    menu_choices.push(Box::new(" No")).unwrap();
    menu_choices.push(Box::new(" Yes")).unwrap();

    state.menu_choices = menu_choices;
    state.menu_prompt = Box::new("Are you sure that you want to use mainnet (real money)?:");
    state.current_screen = ScreenTypes::MenuScreenType;
    state.menu_type = MenuTypes::ConfirmMainnetMenuType;

    true
}

pub fn menu_item_selected(mut state: &mut MMState) -> bool {
    if state
        .menu_choices
        .get(state.menu_hover_index as usize)
        .unwrap()
        .deref()
        .deref()
        .eq(" Yes")
    {
        state.network = Networks::Mainnet;
        true
    } else {
        choose_network_menu::initialize_menu(&mut state)
    }
}
