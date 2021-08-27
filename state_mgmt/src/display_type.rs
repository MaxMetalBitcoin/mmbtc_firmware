#![no_std]

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics_simulator::SimulatorDisplay;

pub type DisplayType = SimulatorDisplay<BinaryColor>;
