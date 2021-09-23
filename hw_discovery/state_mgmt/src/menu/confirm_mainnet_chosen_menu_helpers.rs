// #![no_std]

// extern crate alloc;

// use alloc::boxed::Box;
// use heapless::Vec;

// use crate::menu::choose_network_menu_helpers;
// use crate::menu::menu_screen_state::MenuScreenTypeState;
// use crate::menu::menu_screen_state::MenuTypes;
// use crate::menu::test_signing_menu;
// use crate::networks;

// pub fn init_confirm_mainnet_chosen_menu<'a>() -> MenuScreenTypeState<'a> {
//     let mut choices: Vec<Box<&str>, 20> = Vec::new();

//     choices.push(Box::new(" No")).unwrap();
//     choices.push(Box::new(" Yes")).unwrap();

//     MenuScreenTypeState {
//         prompt: Box::new("Are you sure? This is REAL money:"),
//         choices,
//         hover_index: 0,
//         menu_type: MenuTypes::ConfirmMainnetMenuType,
//     }
// }

// pub fn get_next_menu_on_confirm_mainnet_menu<'a>(
//     menu_screen_types_state: &MenuScreenTypeState,
// ) -> MenuScreenTypeState<'a> {
//     match menu_screen_types_state.hover_index {
//         0 => choose_network_menu_helpers::init_choose_network_menu(),
//         1 => choose_network_menu_helpers::init_choose_network_menu(),
//         // 1 => test_signing_menu::init_test_signing_menu(),
//         _ => panic!("Invalid state - only 2 choices, yes or no"),
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn init_confirm_mainnet_chosen_menu_returns_the_right_content() {
//         let menu = init_confirm_mainnet_chosen_menu();

//         assert_eq!(menu.prompt, "Are you sure? This is REAL money:");
//         assert_eq!(menu.choices.len(), 2);
//         assert_eq!(menu.choices[0], " No");
//         assert_eq!(menu.choices[1], " Yes");
//         assert_eq!(menu.hover_index, 0);
//         assert_eq!(menu.menu_type, MenuTypes::ConfirmMainnetMenuType);
//     }

//     #[test]
//     fn when_on_no_choice_and_hit_enter_send_back_to_choose_network_menu() {
//         let menu = init_confirm_mainnet_chosen_menu();
//         assert_eq!(menu.hover_index, 0);

//         let next_menu = menu.get_next_menu_on_confirm_mainnet_menu();

//         assert_eq!(next_menu.menu_type, MenuTypes::ChooseNetworkMenuType);
//     }
// }
