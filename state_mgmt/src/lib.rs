#![no_std]

use core::fmt::Error;
use core::str;

use heapless::Vec;

// use alloc::vec::Vec;
use embedded_graphics::{
    drawable::Drawable,
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    text_style,
};
use menu::menu_screen_state::MenuScreenState;

pub mod display_type;
pub mod menu;
pub mod mm_state_action;
pub mod networks;

#[derive(Debug)]
pub enum Screen {
    LoadScreen,
    MenuScreen(menu::menu_screen_state::MenuScreenState),
}

#[derive(Debug)]
pub struct MMState {
    pub network: networks::Networks,
    pub current_screen: Screen,
}

impl MMState {
    pub fn new() -> MMState {
        MMState {
            network: networks::Networks::Testnet,
            current_screen: Screen::LoadScreen,
        }
    }

    pub fn update_state(&mut self, action: mm_state_action::MMStateAction) -> bool {
        match &mut self.current_screen {
            Screen::LoadScreen => {
                if action == mm_state_action::MMStateAction::Enter {
                    self.current_screen =
                        Screen::MenuScreen(MenuScreenState::init_choose_network_menu());
                    true
                } else {
                    false
                }
            }
            Screen::MenuScreen(menu_screen_state) => match menu_screen_state.menu_type {
                menu::menu_screen_state::MenuTypes::ChooseNetworkMenuType => {
                    if action == mm_state_action::MMStateAction::Enter {
                        self.network = menu_screen_state.get_network_from_choice();
                        true
                    } else {
                        menu_screen_state.update_state(action)
                    }
                }
                _ => false,
            },
            _ => false,
        }
    }

    pub fn render(
        &self,
        mut display: display_type::DisplayType,
    ) -> Result<display_type::DisplayType, Error> {
        match &self.current_screen {
            Screen::LoadScreen => {
                let text_style = text_style!(
                    font = Font6x8,
                    text_color = BinaryColor::On,
                    background_color = BinaryColor::Off,
                );

                Text::new("Loading...", Point::new(5, 5))
                    .into_styled(text_style)
                    .draw(&mut display)
                    .unwrap();
            }

            Screen::MenuScreen(menu_screen_state) => match menu_screen_state.menu_type {
                menu::menu_screen_state::MenuTypes::ChooseNetworkMenuType => {
                    display = menu_screen_state.render(display).unwrap()
                }
            },

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
