#![no_std]

pub mod choose_network_menu;
pub mod confirm_mainnet_chosen_menu;
pub mod test_signing_menu;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuTypes {
    ChooseNetworkMenuType,
    ConfirmMainnetMenuType,
    TestSignMenuType,
}
