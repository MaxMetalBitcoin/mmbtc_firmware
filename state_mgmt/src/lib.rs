#![no_std]

use core::fmt::Error;

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

pub enum Screen {
    LoadScreen,
    ChooseNetworkScreen,
}

pub struct MMState {
    pub network: &'static str,
    pub currentScreen: Screen,
}

impl MMState {
    pub fn new() -> MMState {
        MMState {
            network: "testnet",
            currentScreen: Screen::ChooseNetworkScreen,
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

                Text::new("Hello World!", Point::new(5, 5))
                    .into_styled(text_style)
                    .draw(&mut display);
            }
            _ => {}
        }

        Ok((display))
    }
}
