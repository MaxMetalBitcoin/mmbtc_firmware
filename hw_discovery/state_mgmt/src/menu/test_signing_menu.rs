#![no_std]

use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::ToString;
use heapless::Vec;

extern crate alloc;

use alloc::boxed::Box;

use core::borrow::Borrow;
use core::str::FromStr;

use crate::bitcoin::{self, util::psbt::PartiallySignedTransaction};
use crate::menu::choose_network_menu_helpers;
use crate::networks;
use crate::screens::menus::MenuTypes;

// pub fn init_test_signing_menu() -> MenuScreenTypeState<'static> {
//     let mut choices: Vec<Box<&str>, 20> = Vec::new();

//     let raw_psbt: &str = "cHNidP8BAHECAAAAAeJQY2VLRtutKgQYFUajEKpjFfl0Uyrm6x23OumDpe/4AQAAAAD/////AkxREgEAAAAAFgAUv6pTgbKHN60CZ+RQn5yOuH6c2WiA8PoCAAAAABYAFJDbOFU0E6zFF/M+g/AKDyqI2iUaAAAAAAABAOsCAAAAAAEBbxqXgEf9DlzcqqNM610s5pL1X258ra6+KJ22etb7HAcBAAAAAAAAAAACACT0AAAAAAAiACC7U1W0iJGhQ6o7CexDh5k36V6v3256xpA9/xmB2BybTFZdDQQAAAAAFgAUKp2ThzhswyM2QHlyvmMB6tQB7V0CSDBFAiEA4Md8RIZYqFdUPsgDyomlzMJL9bJ6Ho23JGTihXtEelgCIAeNXRLyt88SOuuWFVn3IodCE4U5D6DojIHesRmikF28ASEDHYFzMEAxfmfq98eSSnZtUwb1w7mAtHG65y8qiRFNnIkAAAAAAQEfVl0NBAAAAAAWABQqnZOHOGzDIzZAeXK+YwHq1AHtXQEDBAEAAAAAAAA=";
//     let mut bb: &str = "cHNidP8BAHECAAAAAeJQY2VLRtutKgQYFUajEKpjFfl0Uyrm6x23OumDpe/4AQAAAAD/////AkxREgEAAAAAFgAUv6pTgbKHN60CZ+RQn5yOuH6c2WiA8PoCAAAAABYAFJDbOFU0E6zFF/M+g/AKDyqI2iUaAAAAAAABAOsCAAAAAAEBbxqXgEf9DlzcqqNM610s5pL1X258ra6+KJ22etb7HAcBAAAAAAAAAAACACT0AAAAAAAiACC7U1W0iJGhQ6o7CexDh5k36V6v3256xpA9/xmB2BybTFZdDQQAAAAAFgAUKp2ThzhswyM2QHlyvmMB6tQB7V0CSDBFAiEA4Md8RIZYqFdUPsgDyomlzMJL9bJ6Ho23JGTihXtEelgCIAeNXRLyt88SOuuWFVn3IodCE4U5D6DojIHesRmikF28ASEDHYFzMEAxfmfq98eSSnZtUwb1w7mAtHG65y8qiRFNnIkAAAAAAQEfVl0NBAAAAAAWABQqnZOHOGzDIzZAeXK+YwHq1AHtXQEDBAEAAAAAAAA=";

//     choices.push(Box::new(" No")).unwrap();
//     choices.push(Box::new(" Yes")).unwrap();

//     let psbt: PartiallySignedTransaction =
//         bitcoin::util::psbt::PartiallySignedTransaction::from_str(raw_psbt).unwrap();
//     bb = "f";

//     MenuScreenTypeState {
//         prompt: Box::new(psbt.inputs.len().to_string().as_str()),
//         hover_index: 0,
//         menu_type: MenuTypes::TestSignMenuType,
//         choices,
//     }
// }

// pub fn get_next_menu_on_confirm_mainnet_menu<'a>(
//     menu_screen_types_state: &MenuScreenTypeState,
// ) -> MenuScreenTypeState<'a> {
//     match menu_screen_types_state.hover_index {
//         // 0 => choose_network_menu_helpers::init_choose_network_menu(),
//         // 1 => init_confirm_mainnet_chosen_menu(),
//         _ => panic!("Invalid state - only 2 choices, yes or no"),
//     }
// }
