#![no_std]

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

#[derive(Debug, Clone)]
pub enum MenuTypes {
    ChooseNetworkMenuType,
}

#[derive(Debug, Clone)]
pub struct MenuScreenState {
    pub prompt: &'static str,
    pub choices: Vec<&'static str, 20>,
    pub hover_index: u16,
    pub menu_type: MenuTypes,
}

impl MenuScreenState {
    pub fn init_choose_network_menu() -> Self {
        let mut choices: Vec<&'static str, 20> = Vec::new();

        choices.push(" Main").unwrap();
        choices.push(" Testnet").unwrap();
        choices.push(" Signet").unwrap();

        Self {
            prompt: "Choose your network:",
            choices: choices,
            hover_index: 0,
            menu_type: MenuTypes::ChooseNetworkMenuType,
        }
    }

    pub fn get_network_from_choice(&self) -> networks::Networks {
        match self.hover_index {
            0 => Networks::Mainnet,
            1 => Networks::Testnet,
            2 => Networks::Signet,
            _ => panic!("Invalid state - only 3 network choices"),
        }
    }

    pub fn render(
        &self,
        mut display: display_type::DisplayType,
    ) -> Result<display_type::DisplayType, Error> {
        let regular_text_style = text_style!(
            font = Font6x8,
            text_color = BinaryColor::On,
            background_color = BinaryColor::Off,
        );
        let hover_text_style = text_style!(
            font = Font6x8,
            text_color = BinaryColor::Off,
            background_color = BinaryColor::On,
        );
        Text::new(self.prompt, Point::new(5, 5))
            .into_styled(regular_text_style)
            .draw(&mut display)
            .unwrap();

        let font_height = 8;
        let font_spacing_buffer = 4;
        let prompt_spacing_buffer = 4;
        let mut starting_point = 5 + font_height + font_spacing_buffer + prompt_spacing_buffer;
        let mut index = 0;

        for choice in self.choices.iter() {
            if index == self.hover_index {
                Text::new(choice, Point::new(5, starting_point))
                    .into_styled(hover_text_style)
                    .draw(&mut display)
                    .unwrap();
            } else {
                Text::new(choice, Point::new(5, starting_point))
                    .into_styled(regular_text_style)
                    .draw(&mut display)
                    .unwrap();
            }

            index += 1;
            starting_point += font_height + font_spacing_buffer;
        }

        Ok(display)
    }

    pub fn update_state(&mut self, action: mm_state_action::MMStateAction) -> bool {
        match action {
            mm_state_action::MMStateAction::Down => {
                if self.hover_index < self.choices.len() as u16 - 1 {
                    self.hover_index += 1;
                }
                true
            }
            mm_state_action::MMStateAction::Up => {
                if self.hover_index > 0 {
                    self.hover_index -= 1;
                }
                true
            }
            _ => false,
        }
    }
}
