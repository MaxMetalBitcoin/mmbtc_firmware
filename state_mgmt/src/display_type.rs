#![no_std]

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics_simulator::SimulatorDisplay;

// will use compiler instructions to replace this with hw
// display in non-sim compile targets
pub type DisplayType = SimulatorDisplay<BinaryColor>;
