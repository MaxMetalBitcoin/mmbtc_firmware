#![no_std]

// will use compiler instructions to replace this with hw
// display in non-sim compile targets

// this is in both
use embedded_graphics::pixelcolor::BinaryColor;

// sim only
#[cfg(feature = "simulator")]
use embedded_graphics_simulator::SimulatorDisplay;
#[cfg(feature = "simulator")]
pub type DisplayType = SimulatorDisplay<BinaryColor>;

// non-sim only
#[cfg(not(feature = "simulator"))]
use embedded_graphics::mock_display::MockDisplay;
#[cfg(not(feature = "simulator"))]
pub type DisplayType = MockDisplay<BinaryColor>;
