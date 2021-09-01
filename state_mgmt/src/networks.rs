#![no_std]
use core::fmt;

#[derive(Debug)]
pub enum Networks {
    Mainnet,
    Testnet,
    Signet,
}
