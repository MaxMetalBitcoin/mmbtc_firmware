#![no_main]
#![no_std]

use core::ptr;

use cortex_m::iprintln;
use cortex_m_rt::entry;
use f3::hal::{
    gpio::gpioe,
    rcc::RccExt,
    stm32f30x::{self, GPIOE, RCC},
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = stm32f30x::Peripherals::take().unwrap();

    // &mut rcc.ahbenr(|w| w.iopeen().set_bit());

    // let mut rcc = dp.RCC.constrain();

    let mut rcc = unsafe { &*RCC::ptr() };

    rcc.ahbenr.write(|w| w.iopeen().set_bit());

    dp.GPIOE.moder.write(|w| w.moder8().output());

    dp.GPIOE.odr.write(|w| w.odr8().set_bit());

    loop {
        cortex_m::asm::nop();
    }
}
