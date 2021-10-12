#![no_std]

pub mod choose_network_menu;
pub mod confirm_mainnet_chosen_menu;
pub mod list_seeds;
pub mod list_wallets;
pub mod main_menu;
pub mod test_signing_menu;
pub mod where_to_load_transaction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuTypes {
    ChooseNetworkMenuType,
    ConfirmMainnetMenuType,
    TestSignMenuType,
    MainMenuType,
    WhereToLoadTransactionMenuType,
    ListSeedsMenuType,
    ListWalletsMenuType,
}
