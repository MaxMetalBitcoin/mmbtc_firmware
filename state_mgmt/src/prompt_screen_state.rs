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
use heapless::{String, Vec};

#[path = "./display_type.rs"]
mod display_type;
#[path = "./mm_state_action.rs"]
pub mod mm_state_action;

#[derive(Debug, Clone)]
pub struct PromptScreenState {
    pub prompt: &'static str,
    pub choices: Vec<&'static str, 20>,
    pub hover_index: u16,
}

impl PromptScreenState {
    pub fn render(
        &self,
        mut display: display_type::DisplayType,
    ) -> (Result<(display_type::DisplayType), Error>) {
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
            .draw(&mut display);

        let fontHeight = 8;
        let fontSpacingBuffer = 4;
        let prompt_spacing_buffer = 4;
        let mut startingPoint = 5 + fontHeight + fontSpacingBuffer + prompt_spacing_buffer;
        let mut index = 0;

        for choice in self.choices.iter() {
            if (index == self.hover_index) {
                Text::new(choice, Point::new(5, startingPoint))
                    .into_styled(hover_text_style)
                    .draw(&mut display);
            } else {
                Text::new(choice, Point::new(5, startingPoint))
                    .into_styled(regular_text_style)
                    .draw(&mut display);
            }

            index += 1;
            startingPoint += fontHeight + fontSpacingBuffer;
        }

        Ok(display)
    }

    pub fn updateState(&mut self, action: mm_state_action::MMStateAction) -> bool {
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
