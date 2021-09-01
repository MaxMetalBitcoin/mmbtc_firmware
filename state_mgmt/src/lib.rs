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

pub mod display_type;
pub mod menu;
pub mod mm_state_action;

#[derive(Debug)]
pub enum Screen {
    LoadScreen,
    PromptScreen(menu::MenuTypes),
}

#[derive(Debug)]
pub struct MMState {
    pub network: &'static str,
    pub current_screen: Screen,
}

impl MMState {
    pub fn new() -> MMState {
        MMState {
            network: "testnet",
            current_screen: Screen::LoadScreen,
        }
    }

    pub fn update_state(&mut self, action: mm_state_action::MMStateAction) -> bool {
        match &mut self.current_screen {
            Screen::LoadScreen => {
                if action == mm_state_action::MMStateAction::Enter {
                    self.current_screen =
                        Screen::PromptScreen(menu::MenuTypes::ChooseNetworkMenuType(
                            menu::choose_network_menu::ChooseNetworkMenu::init(),
                        ));
                    true
                } else {
                    false
                }
            }
            Screen::PromptScreen(menu_type) => match menu_type {
                menu::MenuTypes::ChooseNetworkMenuType(choose_network_menu) => {
                    choose_network_menu.update_state(action)
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

            Screen::PromptScreen(menu_type) => match menu_type {
                menu::MenuTypes::ChooseNetworkMenuType(choose_network_menu) => {
                    display = choose_network_menu.render(display).unwrap()
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
