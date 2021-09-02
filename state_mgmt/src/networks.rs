#![no_std]
use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Networks {
    Mainnet,
    Testnet,
    Signet,
}
