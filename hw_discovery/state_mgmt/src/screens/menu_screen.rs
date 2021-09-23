#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core::fmt::Error;
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

use super::menus;

pub fn update_state(mut state: &mut MMState, action: mm_state_action::MMStateAction) -> bool {
    match action {
        mm_state_action::MMStateAction::Up => {
            if state.menu_hover_index > 0 {
                state.menu_hover_index = state.menu_hover_index - 1;
                true
            } else {
                false
            }
        }
        mm_state_action::MMStateAction::Down => {
            if state.menu_hover_index < state.menu_choices.len() as u16 - 1 {
                state.menu_hover_index = state.menu_hover_index + 1;
                true
            } else {
                false
            }
        }
        mm_state_action::MMStateAction::Left => false,
        mm_state_action::MMStateAction::Right => false,
        mm_state_action::MMStateAction::Enter => match state.menu_type {
            MenuTypes::ChooseNetworkMenuType => {
                menus::choose_network_menu::menu_item_selected(&mut state)
            }
            MenuTypes::ConfirmMainnetMenuType => {
                menus::confirm_mainnet_chosen_menu::menu_item_selected(&mut state)
            }
            MenuTypes::TestSignMenuType => menus::test_signing_menu::menu_item_selected(&mut state),
        },
    }
}

pub fn render(
    state: &MMState,
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
    Text::new(&state.menu_prompt, Point::new(5, 5))
        .into_styled(regular_text_style)
        .draw(&mut display)
        .unwrap();

    let font_height = 8;
    let font_spacing_buffer = 4;
    let prompt_spacing_buffer = 4;
    let mut starting_point = 5 + font_height + font_spacing_buffer + prompt_spacing_buffer;
    let mut index = 0;

    for choice in state.menu_choices.iter() {
        if index == state.menu_hover_index {
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
