#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::string::ToString;
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

use crate::networks;
use crate::{display_type, networks::Networks};
use crate::{menu::choose_network_menu_helpers, mm_state::MMState};
use crate::{menu::confirm_mainnet_chosen_menu_helpers, mm_state::ScreenTypes};
use crate::{mm_state::MenuTypes, mm_state_action};

use super::choose_network_menu;

pub fn initialize_menu(mut state: &mut MMState) -> bool {
    let mut menu_choices: Vec<Box<String>, 20> = Vec::new();

    let raw_psbt: &str = "cHNidP8BAHECAAAAAeJQY2VLRtutKgQYFUajEKpjFfl0Uyrm6x23OumDpe/4AQAAAAD/////AkxREgEAAAAAFgAUv6pTgbKHN60CZ+RQn5yOuH6c2WiA8PoCAAAAABYAFJDbOFU0E6zFF/M+g/AKDyqI2iUaAAAAAAABAOsCAAAAAAEBbxqXgEf9DlzcqqNM610s5pL1X258ra6+KJ22etb7HAcBAAAAAAAAAAACACT0AAAAAAAiACC7U1W0iJGhQ6o7CexDh5k36V6v3256xpA9/xmB2BybTFZdDQQAAAAAFgAUKp2ThzhswyM2QHlyvmMB6tQB7V0CSDBFAiEA4Md8RIZYqFdUPsgDyomlzMJL9bJ6Ho23JGTihXtEelgCIAeNXRLyt88SOuuWFVn3IodCE4U5D6DojIHesRmikF28ASEDHYFzMEAxfmfq98eSSnZtUwb1w7mAtHG65y8qiRFNnIkAAAAAAQEfVl0NBAAAAAAWABQqnZOHOGzDIzZAeXK+YwHq1AHtXQEDBAEAAAAAAAA=";

    let psbt: PartiallySignedTransaction =
        bitcoin::util::psbt::PartiallySignedTransaction::from_str(raw_psbt).unwrap();

    let count: u32 = (&psbt.outputs).len() as u32;

    let mut data = String::from("data:");
    let _ = write!(data, "{}", count);
    // let data_str: &'static str = &data.as_str();
    // let count_str: &str = count.to_string().as_str();
    // let count_str: &str = psbt.global.version.as_str();
    menu_choices.push(Box::new(" No".to_string())).unwrap();
    menu_choices.push(Box::new(" Yes".to_string())).unwrap();

    state.menu_choices = menu_choices;
    state.menu_prompt = Box::new(data);
    state.current_screen = ScreenTypes::MenuScreenType;
    state.menu_type = MenuTypes::TestSignMenuType;

    true
}

pub fn menu_item_selected(mut state: &mut MMState) -> bool {
    choose_network_menu::initialize_menu(&mut state)
}
