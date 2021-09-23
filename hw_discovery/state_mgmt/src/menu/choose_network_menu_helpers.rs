// #![no_std]

// extern crate alloc;

// use alloc::boxed::Box;
// use heapless::Vec;

// use crate::menu::confirm_mainnet_chosen_menu_helpers;
// // use crate::menu::menu_screen_state::MenuScreenTypeState;
// use crate::menu::menu_screen_state::MenuTypes;
// use crate::networks;

// pub fn init_choose_network_menu<'a>() -> MenuScreenTypeState<'a> {
//     let mut choices: Vec<Box<&str>, 20> = Vec::new();

//     choices.push(Box::new(" Main")).unwrap();
//     choices.push(Box::new(" Testnet")).unwrap();
//     choices.push(Box::new(" Signet")).unwrap();

//     MenuScreenTypeState {
//         prompt: Box::new("Choose your network:"),
//         choices,
//         hover_index: 0,
//         menu_type: MenuTypes::ChooseNetworkMenuType,
//     }
// }

// pub fn get_network_from_choice(
//     menu_screen_types_state: &MenuScreenTypeState,
// ) -> networks::Networks {
//     match menu_screen_types_state.hover_index {
//         0 => networks::Networks::Mainnet,
//         1 => networks::Networks::Testnet,
//         2 => networks::Networks::Signet,
//         _ => panic!("Invalid state - only 3 network choices"),
//     }
// }

// pub fn get_next_menu_on_choose_network_menu<'a>(
//     menu_screen_types_state: &MenuScreenTypeState,
// ) -> MenuScreenTypeState<'a> {
//     match menu_screen_types_state.hover_index {
//         0 => confirm_mainnet_chosen_menu_helpers::init_confirm_mainnet_chosen_menu(),
//         1 => confirm_mainnet_chosen_menu_helpers::init_confirm_mainnet_chosen_menu(),
//         2 => confirm_mainnet_chosen_menu_helpers::init_confirm_mainnet_chosen_menu(),
//         _ => panic!("Invalid state - only 3 network choices"),
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::mm_state_action::MMStateAction;

//     #[test]
//     fn init_choose_network_menu_returns_the_right_content() {
//         let menu = init_choose_network_menu();

//         assert_eq!(menu.prompt, "Choose your network:");
//         assert_eq!(menu.choices.len(), 3);
//         assert_eq!(menu.choices[0], " Main");
//         assert_eq!(menu.choices[1], " Testnet");
//         assert_eq!(menu.choices[2], " Signet");
//         assert_eq!(menu.hover_index, 0);
//         assert_eq!(menu.menu_type, MenuTypes::ChooseNetworkMenuType);
//     }

//     #[test]
//     fn get_network_from_choice_all_choices() {
//         let mut menu = init_choose_network_menu();

//         assert_eq!(menu.hover_index, 0);
//         let network = menu.get_network_from_choice();
//         assert_eq!(network, networks::Networks::Mainnet);

//         menu.update_state(MMStateAction::Down);
//         assert_eq!(menu.hover_index, 1);
//         let network = menu.get_network_from_choice();
//         assert_eq!(network, networks::Networks::Testnet);

//         menu.update_state(MMStateAction::Down);
//         assert_eq!(menu.hover_index, 2);
//         let network = menu.get_network_from_choice();
//         assert_eq!(network, networks::Networks::Signet);
//     }

//     #[test]
//     fn when_on_mainnet_choice_and_hit_enter_go_to_confirm_page() {
//         let menu = init_choose_network_menu();
//         assert_eq!(menu.hover_index, 0);

//         let next_menu = menu.get_next_menu_on_choose_network_menu();

//         assert_eq!(next_menu.menu_type, MenuTypes::ConfirmMainnetMenuType);
//     }
// }
