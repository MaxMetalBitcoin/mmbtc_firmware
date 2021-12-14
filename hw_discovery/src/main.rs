#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use cortex_m::Peripherals;
use cortex_m::delay::Delay;
use cortex_m::peripheral::SYST;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::prelude::{_embedded_hal_blocking_delay_DelayMs, _embedded_hal_blocking_spi_Write};
use f3::hal::rcc::Rcc;
use core::borrow::Borrow;
use core::marker::PhantomData;
use core::ops::Deref;
use core::{alloc::Layout, pin::Pin};
use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m::{asm::nop, iprintln};
use cortex_m_rt::{entry, exception, ExceptionFrame};

use embedded_hal::{digital::v2::OutputPin, spi::{Mode, Phase, Polarity}, blocking::delay::DelayMs, };
use embedded_sdmmc::{DirEntry, TimeSource, Timestamp};
use f3::hal::{gpio::{gpioa::{PA0, PA1, PA4, PA5, PA6, PA7}, gpioe, GpioExt, Output, PushPull, AF5},  i2c::SdaPin, prelude::_stm32f30x_hal_flash_FlashExt,  rcc::{Clocks, RccExt, APB2}, spi::{MisoPin, MosiPin, SckPin, Spi}, stm32f30x::{self, FLASH, GPIOA, RCC, SPI1 }, time::Hertz};
use heapless::String;
use panic_itm; // panic handler

use embedded_graphics::{
  fonts::{Font8x16, Text},
  pixelcolor::Rgb565,
  prelude::*,
  primitives::{Circle, Rectangle},
  style::{PrimitiveStyle, TextStyle},
};


// use embedded_graphics_core::{primitives::rectangle::Rectangle, draw_target::DrawTarget, pixelcolor::Rgb565, geometry::{Size, Point}};

use ili9341::{Ili9341, Orientation};
use display_interface;

// use state_mgmt;

use alloc_cortex_m::CortexMHeap;
// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024 * 40; // 40 KB .. the RAM size on f3discovery

// struct my_delay;

// impl DelayMs<u16> for my_delay {
//   fn delay_ms(&mut self, ms: u16) {
//     delay(ms.into())
//   }
// }

struct SPIWrapper {
  spi: Box<Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>)>>,
}

impl display_interface::WriteOnlyDataCommand for SPIWrapper {
  fn send_commands(&mut self, cmd: display_interface::DataFormat<'_>) -> Result<(), display_interface::DisplayError> {
    match cmd {
      display_interface::DataFormat::U8(d) => {
        self.spi.write(d);

      },
      display_interface::DataFormat::U16(d) => {
        for item in d.iter() {
          self.spi.write(&item.to_be_bytes());
        }
      },
      display_interface::DataFormat::U16BE(d) => {
        for item in d.iter() {
          self.spi.write(&item.to_be_bytes());
        }
      },
      display_interface::DataFormat::U16LE(d) => {
        for item in d.iter() {
          self.spi.write(&item.to_be_bytes());
        }
      },
      display_interface::DataFormat::U8Iter(d) => {
        for item in d {
          self.spi.write(&item.to_be_bytes());
        }
      },
      display_interface::DataFormat::U16BEIter(d) => {
        for item in d {
          self.spi.write(&item.to_be_bytes());
        }
      },
      display_interface::DataFormat::U16LEIter(d) => {
        for item in d {
          self.spi.write(&item.to_be_bytes());
        }
      },
      _ => {},
    }
    Ok(())
  }
  fn send_data(&mut self, buf: display_interface::DataFormat<'_>) -> Result<(), display_interface::DisplayError> {
    match buf {
      display_interface::DataFormat::U8(d) => {
        self.spi.write(d);

      },
      display_interface::DataFormat::U16(d) => {
        for item in d.iter() {
          self.spi.write(&item.to_be_bytes());
        }
      },
      display_interface::DataFormat::U16BE(d) => {
        for item in d.iter() {
          self.spi.write(&item.to_be_bytes());
        }
      },
      display_interface::DataFormat::U16LE(d) => {
        for item in d.iter() {
          self.spi.write(&item.to_be_bytes());
        }
      },
      display_interface::DataFormat::U8Iter(d) => {
        for item in d {
          self.spi.write(&item.to_be_bytes());
        }
      },
      display_interface::DataFormat::U16BEIter(d) => {
        for item in d {
          self.spi.write(&item.to_be_bytes());
        }
      },
      display_interface::DataFormat::U16LEIter(d) => {
        for item in d {
          self.spi.write(&item.to_be_bytes());
        }
      },
      _ => {},
    }
    Ok(())
  }
}

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = stm32f30x::Peripherals::take().unwrap();

    let mut rcc_2 = dp.RCC.constrain();
    let mut flash_2 = dp.FLASH.constrain();

    // let clocks = rcc_2.cfgr.hclk(Hertz(8_000_000)).sysclk(Hertz(48_000_000)).pclk1(Hertz(24_000_000)).freeze(&mut flash_2.acr);
    let clocks = rcc_2.cfgr.freeze(&mut flash_2.acr);


    // dp.GPIOA.moder.write(|w| {
    //     w.moder0().output();
    //     w.moder1().output();
    //     w.moder4().output();
    //     w.moder5().output();
    //     w.moder6().output();
    //     w.moder7().output()
    // });

    let mut gpioa = dp.GPIOA.split(&mut rcc_2.ahb);


    let slave_select = gpioa
        .pa4
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let s_clock = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let s_miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let s_mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let s_dc = gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let s_reset = gpioa.pa1.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let spi: Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>)> = Spi::spi1(
      dp.SPI1,
      (s_clock, s_miso, s_mosi),
      Mode {
          polarity: Polarity::IdleLow,
          phase: Phase::CaptureOnFirstTransition,
      },
      Hertz(8_000_000),
      clocks,
      &mut rcc_2.apb2,
    );

    // let screen_card_spi = SPIWrapper {
    //   spi: Box::new(spi),
    // };

    let mut delayy = Delay::new(cp.SYST, clocks.sysclk().0);


    match Ili9341::new_spi(spi, slave_select, s_dc, s_reset, &mut delayy) {
      Ok(mut display) => {
          // screen.fill_solid(&Rectangle {size: Size {height: 100, width: 100}, top_left: Point {x: 0, y: 0}}, Rgb565::new(120,120,120));
          display.set_orientation(Orientation::LandscapeFlipped);

          let c = Circle::new(Point::new(20, 20), 50)
              .into_styled(PrimitiveStyle::with_fill(Rgb565::RED));
          let t = Text::new("Hello Rust!", Point::new(20, 16))
              .into_styled(TextStyle::new(Font8x16, Rgb565::GREEN));
          let r = Rectangle::new(Point::new(0, 0), Point::new(320, 240))
              .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK));
  
          r.draw(&mut display).expect("draw failed");
          c.draw(&mut display).expect("draw failed");
          t.draw(&mut display).expect("draw failed");

              cortex_m::asm::nop();
      }
      Err(err) => {
        cortex_m::asm::nop();
      }
    }

    loop {
        cortex_m::asm::nop();
    }
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    loop {
        cortex_m::asm::bkpt();
        cortex_m::asm::nop();
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    loop {
        cortex_m::asm::bkpt();
        cortex_m::asm::nop();
    }
}
