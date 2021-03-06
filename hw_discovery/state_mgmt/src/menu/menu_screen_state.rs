// #![no_std]

// extern crate alloc;

// use alloc::boxed::Box;
// use core::fmt::Error;
// use embedded_graphics::{
//     drawable::Drawable,
//     fonts::{Font6x8, Text},
//     pixelcolor::BinaryColor,
//     prelude::*,
//     text_style,
// };
// use heapless::Vec;

// use crate::menu::choose_network_menu_helpers;
// use crate::menu::confirm_mainnet_chosen_menu_helpers;
// use crate::mm_state_action;
// use crate::networks;
// use crate::{display_type, networks::Networks};

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum MenuTypes {
//     ChooseNetworkMenuType,
//     ConfirmMainnetMenuType,
//     TestSignMenuType,
// }

// #[derive(Debug, Clone, Copy)]
// pub struct MenuScreenTypeState<'a> {
//     pub prompt: Box<&'a str>,
//     pub choices: Vec<Box<&'a str>, 20>,
//     pub hover_index: u16,
//     pub menu_type: MenuTypes,
// }

// impl<'a> MenuScreenTypeState<'a> {
//     // choose network menu functions - begin
//     pub fn init_choose_network_menu() -> Self {
//         choose_network_menu_helpers::init_choose_network_menu()
//     }

//     pub fn get_network_from_choice(&self) -> networks::Networks {
//         choose_network_menu_helpers::get_network_from_choice(self)
//     }

//     pub fn get_next_menu_on_choose_network_menu(&self) -> MenuScreenTypeState {
//         choose_network_menu_helpers::get_next_menu_on_choose_network_menu(self)
//     }
//     // choose network menu functions - end

//     // confirm mainnet chosen - begin
//     pub fn init_confirm_mainnet_chosen_menu() -> Self {
//         confirm_mainnet_chosen_menu_helpers::init_confirm_mainnet_chosen_menu()
//     }

//     pub fn get_next_menu_on_confirm_mainnet_menu(&self) -> Self {
//         confirm_mainnet_chosen_menu_helpers::get_next_menu_on_confirm_mainnet_menu(self)
//     }
//     // confirm mainnet chosen - end

//     pub fn render(
//         &self,
//         mut display: display_type::DisplayType,
//     ) -> Result<display_type::DisplayType, Error> {
//         let regular_text_style = text_style!(
//             font = Font6x8,
//             text_color = BinaryColor::On,
//             background_color = BinaryColor::Off,
//         );
//         let hover_text_style = text_style!(
//             font = Font6x8,
//             text_color = BinaryColor::Off,
//             background_color = BinaryColor::On,
//         );
//         Text::new(&self.prompt, Point::new(5, 5))
//             .into_styled(regular_text_style)
//             .draw(&mut display)
//             .unwrap();

//         let font_height = 8;
//         let font_spacing_buffer = 4;
//         let prompt_spacing_buffer = 4;
//         let mut starting_point = 5 + font_height + font_spacing_buffer + prompt_spacing_buffer;
//         let mut index = 0;

//         for choice in self.choices.iter() {
//             if index == self.hover_index {
//                 Text::new(choice, Point::new(5, starting_point))
//                     .into_styled(hover_text_style)
//                     .draw(&mut display)
//                     .unwrap();
//             } else {
//                 Text::new(choice, Point::new(5, starting_point))
//                     .into_styled(regular_text_style)
//                     .draw(&mut display)
//                     .unwrap();
//             }

//             index += 1;
//             starting_point += font_height + font_spacing_buffer;
//         }

//         Ok(display)
//     }

//     pub fn update_state(&mut self, action: mm_state_action::MMStateAction) -> bool {
//         match action {
//             mm_state_action::MMStateAction::Down => {
//                 if self.hover_index < self.choices.len() as u16 - 1 {
//                     self.hover_index += 1;
//                 }
//                 true
//             }
//             mm_state_action::MMStateAction::Up => {
//                 if self.hover_index > 0 {
//                     self.hover_index -= 1;
//                 }
//                 true
//             }
//             _ => false,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::mm_state_action::MMStateAction;

//     #[test]
//     fn test_update_state_doesnt_go_past_ends() {
//         let mut menu = choose_network_menu_helpers::init_choose_network_menu();

//         assert_eq!(menu.choices.len(), 3); // so hover_index of 2 is max
//         assert_eq!(menu.hover_index, 0);

//         // test top case - trying to go up past
//         menu.update_state(MMStateAction::Up);
//         assert_eq!(menu.hover_index, 0);

//         menu.update_state(MMStateAction::Down);
//         assert_eq!(menu.hover_index, 1);

//         menu.update_state(MMStateAction::Down);
//         assert_eq!(menu.hover_index, 2);

//         // test bottom case - trying to go down past
//         menu.update_state(MMStateAction::Down);
//         assert_eq!(menu.hover_index, 2);
//     }
// }
