#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core::fmt::Error;
use embedded_graphics::{
    drawable::Drawable,
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    text_style,
};
use heapless::Vec;

use crate::mm_state_action;
use crate::networks;
use crate::{display_type, networks::Networks};
use crate::{menu::choose_network_menu_helpers, mm_state::MMState};
use crate::{menu::confirm_mainnet_chosen_menu_helpers, mm_state::ScreenTypes};

use super::menus;

pub fn update_state(mut state: &mut MMState, action: mm_state_action::MMStateAction) -> bool {
    if action.eq(&mm_state_action::MMStateAction::Enter) {
        menus::choose_network_menu::initialize_menu(&mut state)
    } else {
        false
    }
}

pub fn render(
    state: &MMState,
    mut display: display_type::DisplayType,
) -> Result<display_type::DisplayType, Error> {
    let text_style = text_style!(
        font = Font6x8,
        text_color = BinaryColor::On,
        background_color = BinaryColor::Off,
    );

    Text::new("Loading...", Point::new(5, 5))
        .into_styled(text_style)
        .draw(&mut display)
        .unwrap();

    let size = bitcoin::secp256k1::Secp256k1::preallocate_size();

    let zeroed = bitcoin::secp256k1::ffi::types::AlignedType::zeroed();
    let mut buf_ful = alloc::vec![zeroed; size];
    let secp = bitcoin::secp256k1::Secp256k1::preallocated_new(&mut buf_ful).unwrap();

    Text::new(
        alloc::format!("{}", state.private_key).as_str(),
        Point::new(5, 20),
    )
    .into_styled(text_style)
    .draw(&mut display)
    .unwrap();

    let pubkey = state.private_key.public_key(&secp);
    let address = bitcoin::Address::p2wpkh(&pubkey, bitcoin::Network::Bitcoin).unwrap();

    Text::new(alloc::format!("{}", address).as_str(), Point::new(5, 30))
        .into_styled(text_style)
        .draw(&mut display)
        .unwrap();
    Ok(display)
}
