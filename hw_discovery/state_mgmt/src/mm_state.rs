#![no_std]

use core::borrow::BorrowMut;
use core::str;
use core::{borrow::Borrow, fmt::Error};

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use bitcoin::secp256k1::{AllPreallocated, Secp256k1};
use heapless::Vec;

// use alloc::vec::Vec;
use embedded_graphics::{
    drawable::Drawable,
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    text_style,
};

use crate::bitcoin;
use crate::display_type;
use crate::menu::{self, menu_screen_state};
use crate::mm_state_action;
use crate::networks;
use crate::screens;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuTypes {
    ChooseNetworkMenuType,
    ConfirmMainnetMenuType,
    TestSignMenuType,
}

#[derive(Debug, Clone)]
pub enum ScreenTypes {
    LoadScreenType,
    MenuScreenType,
}

#[derive(Debug)]
pub struct MMState {
    pub current_screen: ScreenTypes,
    pub network: networks::Networks,
    pub private_key: bitcoin::PrivateKey,
    // pub current_screen: menu::screen_types::ScreenTypes<'a>,
    // pub private_key: bitcoin::PrivateKey,
    pub menu_prompt: Box<String>,
    pub menu_choices: Vec<Box<String>, 20>,
    pub menu_hover_index: u16,
    pub menu_type: MenuTypes,
}

pub fn new() -> MMState {
    let raw = "L1HKVVLHXiUhecWnwFYF6L3shkf1E12HUmuZTESvBXUdx3yqVP1D";
    let pk = bitcoin::PrivateKey::from_wif(raw).unwrap();

    MMState {
        current_screen: ScreenTypes::LoadScreenType,
        network: networks::Networks::Testnet,
        private_key: pk,
        menu_prompt: Box::new("Choose you network:".to_string()),
        menu_choices: Vec::new(),
        menu_hover_index: 0,
        menu_type: MenuTypes::ChooseNetworkMenuType,
    }
}

pub fn update_state(mut state: &mut MMState, action: mm_state_action::MMStateAction) -> bool {
    match state.current_screen {
        ScreenTypes::LoadScreenType => screens::load_screen::update_state(&mut state, action),
        ScreenTypes::MenuScreenType => screens::menu_screen::update_state(&mut state, action),
    }
}

pub fn render(
    state: &MMState,
    mut display: display_type::DisplayType,
) -> Result<display_type::DisplayType, Error> {
    match state.current_screen {
        ScreenTypes::LoadScreenType => screens::load_screen::render(&state, display),
        ScreenTypes::MenuScreenType => screens::menu_screen::render(&state, display),
    }
}
