#![no_std]

use core::array;
use core::fmt::Error;
use core::str;

use heapless::{String, Vec};

// use alloc::vec::Vec;
use embedded_graphics::{
    drawable::Drawable,
    fonts::{Font6x8, Text},
    mock_display::MockDisplay,
    pixelcolor::{BinaryColor, PixelColor},
    prelude::*,
    primitives::{Circle, Line, Rectangle},
    text_style, DrawTarget,
};

pub mod display_type;
pub mod mm_state_action;
pub mod prompt_screen_state;

#[derive(Debug)]
pub enum Screen {
    LoadScreen,
    ChooseNetworkScreen,
    PromptScreen(prompt_screen_state::PromptScreenState),
}

#[derive(Debug)]
pub struct MMState {
    pub network: &'static str,
    pub currentScreen: Screen,
}

impl MMState {
    pub fn new() -> MMState {
        MMState {
            network: "testnet",
            currentScreen: Screen::LoadScreen,
        }
    }

    pub fn updateState(&mut self, action: mm_state_action::MMStateAction) -> bool {
        match &mut self.currentScreen {
            Screen::LoadScreen => {
                if (action == mm_state_action::MMStateAction::Enter) {
                    let mut choices: Vec<&'static str, 20> = Vec::new();
                    choices.push(" Main");
                    choices.push(" Testnet");
                    choices.push(" Signet");
                    self.currentScreen =
                        Screen::PromptScreen(prompt_screen_state::PromptScreenState {
                            prompt: "Choose your network:",
                            choices: choices,
                            hover_index: 0,
                        });
                    true
                } else {
                    false
                }
            }
            Screen::PromptScreen(prompt_screen_state) => prompt_screen_state.updateState(action),
            _ => false,
        }
    }

    pub fn render(
        &self,
        mut display: display_type::DisplayType,
    ) -> Result<(display_type::DisplayType), Error> {
        match &self.currentScreen {
            Screen::LoadScreen => {
                let text_style = text_style!(
                    font = Font6x8,
                    text_color = BinaryColor::On,
                    background_color = BinaryColor::Off,
                );

                Text::new("Loading...", Point::new(5, 5))
                    .into_styled(text_style)
                    .draw(&mut display);
            }

            Screen::PromptScreen(prompt_screen_state) => {
                display = prompt_screen_state.render(display).unwrap()
            }

            _ => {
                let text_style = text_style!(
                    font = Font6x8,
                    text_color = BinaryColor::On,
                    background_color = BinaryColor::Off,
                );
                Text::new("Invalid state", Point::new(5, 5))
                    .into_styled(text_style)
                    .draw(&mut display);
            }
        }

        Ok((display))
    }
}
