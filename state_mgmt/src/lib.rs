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

use embedded_graphics_simulator::SimulatorDisplay;

#[derive(Debug, Clone, Copy)]
pub struct PromptScreenState {
    pub prompt: &'static str,
    pub choices: &'static Vec<&'static str, 20>,
}

#[derive(Debug)]
pub enum Screen {
    LoadScreen,
    ChooseNetworkScreen,
    PromptScreen(PromptScreenState),
}

#[derive(Debug)]
pub struct MMState {
    pub network: &'static str,
    pub currentScreen: Screen,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MMStateAction {
    Up,
    Down,
    Left,
    Right,
    Enter,
}

impl MMState {
    pub fn new() -> MMState {
        MMState {
            network: "testnet",
            currentScreen: Screen::LoadScreen,
        }
    }

    pub fn updateState(&mut self, action: MMStateAction) -> bool {
        match self.currentScreen {
            Screen::LoadScreen => {
                if (action == MMStateAction::Enter) {
                    static choices: &'static Vec<&'static str, 20> = &Vec::new();
                    self.currentScreen = Screen::PromptScreen(PromptScreenState {
                        prompt: "Choose your network:",
                        choices: &choices,
                    });
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn render(
        &self,
        mut display: SimulatorDisplay<BinaryColor>,
    ) -> Result<(SimulatorDisplay<BinaryColor>), Error> {
        match self.currentScreen {
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

            Screen::PromptScreen(promptScreenState) => {
                let text_style = text_style!(
                    font = Font6x8,
                    text_color = BinaryColor::On,
                    background_color = BinaryColor::Off,
                );
                Text::new(promptScreenState.prompt, Point::new(5, 5))
                    .into_styled(text_style)
                    .draw(&mut display);
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
