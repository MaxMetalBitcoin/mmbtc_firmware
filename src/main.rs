#![no_main]
#![no_std]

use core::{
    borrow::{Borrow, BorrowMut},
    marker::PhantomData,
    ptr,
};

use cortex_m::iprintln;
use cortex_m_rt::entry;
use embedded_hal::spi::{Mode, Phase, Polarity};
use embedded_sdmmc::{TimeSource, Timestamp};
use f3::hal::{
    gpio::{gpioa, gpioe, GpioExt},
    prelude::_stm32f30x_hal_flash_FlashExt,
    rcc::{Clocks, RccExt, APB2},
    spi::Spi,
    stm32f30x::{self, FLASH, GPIOA, RCC, SPI1},
    time::Hertz,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = stm32f30x::Peripherals::take().unwrap();

    // &mut rcc.ahbenr(|w| w.iopeen().set_bit())

    let mut rcc_2 = dp.RCC.constrain();
    let mut flash_2 = dp.FLASH.constrain();

    let mut flash = unsafe { &*FLASH::ptr() };
    let mut rcc = unsafe { &*RCC::ptr() };

    let clocks = rcc_2.cfgr.freeze(&mut flash_2.acr);

    struct MyTimeSouce;
    impl TimeSource for MyTimeSouce {
        fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
            Timestamp {
                /// Add 1970 to this file to get the calendar year
                year_since_1970: 51,
                /// Add one to this value to get the calendar month
                zero_indexed_month: 7,
                /// Add one to this value to get the calendar day
                zero_indexed_day: 14,
                /// The number of hours past midnight
                hours: 22,
                /// The number of minutes past the hour
                minutes: 27,
                /// The number of seconds past the minute
                seconds: 50,
            }
        }
    }

    rcc.ahbenr.write(|w| w.iopaen().set_bit());

    // dp.GPIOA.moder.write(|w| {
    //     w.moder4().();
    //     w.moder5().output();
    //     w.moder6().output();
    //     w.moder7().output()
    // });

    let mut gpioa = dp.GPIOA.split(&mut rcc_2.ahb);

    // let spi1: SPI1 = SPI1::
    // let mut apb2: APB2 = APB2 { _0: () };

    let slave_select = gpioa
        .pa4
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let s_clock = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let s_miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let s_mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    let sd_card_spi = Spi::spi1(
        dp.SPI1,
        (s_clock, s_miso, s_mosi),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        Hertz(50_000),
        clocks,
        &mut rcc_2.apb2,
    );

    let sd_mmc_spi = embedded_sdmmc::SdMmcSpi::new(sd_card_spi, slave_select);

    let mut cont = embedded_sdmmc::Controller::new(sd_mmc_spi, MyTimeSouce {});

    // dp.GPIOE.odr.write(|w| w.odr8().set_bit());

    // write_to.write_u8(100);

    match cont.device().init() {
        Ok(_) => {
            // write!(uart, "OK!\nCard size...").unwrap();
            match cont.device().card_size_bytes() {
                Ok(size) => iprintln!(&mut cp.ITM.stim[0], "{}", size),
                Err(e) => iprintln!(&mut cp.ITM.stim[0], "Err here"),
            }
            // write!(uart, "Volume 0...").unwrap();
            // match cont.get_volume(embedded_sdmmc::VolumeIdx(0)) {
            //     Ok(v) => writeln!(uart, "{:?}", v).unwrap(),
            //     Err(e) => writeln!(uart, "Err: {:?}", e).unwrap(),
            // }
        }
        Err(e) => iprintln!(&mut cp.ITM.stim[0], "Err down here"),
    }

    loop {
        cortex_m::asm::nop();
    }
}
