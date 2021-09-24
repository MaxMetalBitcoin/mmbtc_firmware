#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::string::ToString;
use bitcoin::util::psbt::Map;
use bitcoin::util::psbt::{Global, Input, PartiallySignedTransaction};
use core::borrow::Borrow;
use core::fmt::Write;
use core::ops::Not;
use core::str::FromStr;
use core::{fmt::Error, ops::Deref};
use embedded_graphics::{
    drawable::Drawable,
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    text_style,
};
use heapless::Vec;

use super::MenuTypes;
use crate::mm_state_action;
use crate::networks;
use crate::{display_type, networks::Networks};
use crate::{menu::choose_network_menu_helpers, mm_state::MMState};
use crate::{menu::confirm_mainnet_chosen_menu_helpers, mm_state::ScreenTypes};

use super::choose_network_menu;

pub fn initialize_menu(mut state: &mut MMState) -> bool {
    let mut menu_choices: Vec<Box<String>, 20> = Vec::new();

    let raw_psbt: &str = "cHNidP8BAFUCAAAAASeaIyOl37UfxF8iD6WLD8E+HjNCeSqF1+Ns1jM7XLw5AAAAAAD/////AaBa6gsAAAAAGXapFP/pwAYQl8w7Y28ssEYPpPxCfStFiKwAAAAAAAEBIJVe6gsAAAAAF6kUY0UgD2jRieGtwN8cTRbqjxTA2+uHIgIDsTQcy6doO2r08SOM1ul+cWfVafrEfx5I1HVBhENVvUZGMEMCIAQktY7/qqaU4VWepck7v9SokGQiQFXN8HC2dxRpRC0HAh9cjrD+plFtYLisszrWTt5g6Hhb+zqpS5m9+GFR25qaAQEDBAMAAAABBCIAIHcf0YrUWWZt1J89Vk49vEL0yEd042CtoWgWqO1IjVaBAQVHUiEDsTQcy6doO2r08SOM1ul+cWfVafrEfx5I1HVBhENVvUYhA95V0eHayAXj+KWMH7+blMAvPbqv4Sf+/KSZXyb4IIO9Uq4iBgOxNBzLp2g7avTxI4zW6X5xZ9Vp+sR/HkjUdUGEQ1W9RhC0prpnAAAAgAAAAIAEAACAIgYD3lXR4drIBeP4pYwfv5uUwC89uq/hJ/78pJlfJvggg70QtKa6ZwAAAIAAAACABQAAgAAA";

    let psbt: PartiallySignedTransaction =
        bitcoin::util::psbt::PartiallySignedTransaction::from_str(raw_psbt).unwrap();

    let count: u32 = (&psbt.outputs).len() as u32;

    let partial_sigs = psbt.inputs.get(0).unwrap().clone().bip32_derivation;

    for (pubk, (f, dpath)) in &partial_sigs {
        menu_choices.push(Box::new(pubk.to_string())).unwrap();
        menu_choices.push(Box::new(f.to_string())).unwrap();
        menu_choices.push(Box::new(dpath.to_string())).unwrap();
        // menu_choices.push(Box::new(String::from_utf8(sig.to_vec()).unwrap()));
    }

    // let mut str = deriv_paths.unwrap().to_string();
    let mut i = 0;

    let mut data = String::from("data:");
    let _ = write!(data, "{}", count);
    // let data_str: &'static str = &data.as_str();
    // let count_str: &str = count.to_string().as_str();
    // let count_str: &str = psbt.global.version.as_str();
    menu_choices.push(Box::new(" No".to_string())).unwrap();
    menu_choices.push(Box::new(" Yes".to_string())).unwrap();

    state.current_screen = ScreenTypes::MenuScreenType;
    state.menu_prompt = Box::new("header".to_string());
    state.menu_choices = menu_choices;
    state.menu_hover_index = 0;
    state.menu_type = MenuTypes::TestSignMenuType;

    true
}

pub fn menu_item_selected(mut state: &mut MMState) -> bool {
    choose_network_menu::initialize_menu(&mut state)
}
