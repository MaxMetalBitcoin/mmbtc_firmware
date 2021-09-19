#![no_std]

use core::str;
use core::{borrow::Borrow, fmt::Error};

extern crate alloc;
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
use crate::menu;
use crate::mm_state_action;
use crate::networks;

#[derive(Debug)]
pub struct MMState {
    pub network: networks::Networks,
    pub current_screen: menu::screen_types::ScreenTypes,
    pub private_key: bitcoin::PrivateKey,
}

impl MMState {
    pub fn new() -> MMState {
        // hprintln!("secp buf size {}", size * 16).unwrap();

        // Load a private key
        let raw = "L1HKVVLHXiUhecWnwFYF6L3shkf1E12HUmuZTESvBXUdx3yqVP1D";
        let pk = bitcoin::PrivateKey::from_wif(raw).unwrap();
        // hprintln!("Seed WIF: {}", pk).unwrap();

        MMState {
            network: networks::Networks::Testnet,
            current_screen: menu::screen_types::ScreenTypes::LoadScreenTypes,
            private_key: pk,
        }
    }

    pub fn update_state(&mut self, action: mm_state_action::MMStateAction) -> bool {
        match &mut self.current_screen {
            menu::screen_types::ScreenTypes::LoadScreenTypes => {
                if action == mm_state_action::MMStateAction::Enter {
                    self.current_screen = menu::screen_types::ScreenTypes::MenuScreenTypes(
                        menu::menu_screen_state::MenuScreenTypesState::init_choose_network_menu(),
                    );
                    true
                } else {
                    false
                }
            }
            menu::screen_types::ScreenTypes::MenuScreenTypes(menu_screen_state) => {
                match menu_screen_state.menu_type {
                    menu::menu_screen_state::MenuTypes::ChooseNetworkMenuType => {
                        if action == mm_state_action::MMStateAction::Enter {
                            self.network = menu_screen_state.get_network_from_choice();
                            self.current_screen = menu::screen_types::ScreenTypes::MenuScreenTypes(
                                menu_screen_state.get_next_menu_on_choose_network_menu(),
                            );

                            true
                        } else {
                            menu_screen_state.update_state(action)
                        }
                    }
                    menu::menu_screen_state::MenuTypes::ConfirmMainnetMenuType => {
                        if action == mm_state_action::MMStateAction::Enter {
                            self.current_screen = menu::screen_types::ScreenTypes::MenuScreenTypes(
                                menu_screen_state.get_next_menu_on_confirm_mainnet_menu(),
                            );
                            true
                        } else {
                            menu_screen_state.update_state(action)
                        }
                    }
                }
            }
            _ => false,
        }
    }

    pub fn render(
        &self,
        mut display: display_type::DisplayType,
    ) -> Result<display_type::DisplayType, Error> {
        match &self.current_screen {
            menu::screen_types::ScreenTypes::LoadScreenTypes => {
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
                    alloc::format!("{}", self.private_key).as_str(),
                    Point::new(5, 20),
                )
                .into_styled(text_style)
                .draw(&mut display)
                .unwrap();

                let pubkey = self.private_key.public_key(&secp);
                let address = bitcoin::Address::p2wpkh(&pubkey, bitcoin::Network::Bitcoin).unwrap();

                Text::new(alloc::format!("{}", address).as_str(), Point::new(5, 30))
                    .into_styled(text_style)
                    .draw(&mut display)
                    .unwrap();
            }

            menu::screen_types::ScreenTypes::MenuScreenTypes(menu_screen_state) => {
                match menu_screen_state.menu_type {
                    menu::menu_screen_state::MenuTypes::ChooseNetworkMenuType => {
                        display = menu_screen_state.render(display).unwrap()
                    }
                    menu::menu_screen_state::MenuTypes::ConfirmMainnetMenuType => {
                        display = menu_screen_state.render(display).unwrap()
                    }
                }
            }

            _ => {
                let text_style = text_style!(
                    font = Font6x8,
                    text_color = BinaryColor::On,
                    background_color = BinaryColor::Off,
                );
                Text::new("Invalid state", Point::new(5, 5))
                    .into_styled(text_style)
                    .draw(&mut display)
                    .unwrap();
            }
        }

        Ok(display)
    }
}
