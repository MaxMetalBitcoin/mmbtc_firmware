#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::alloc::Layout;
use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m::iprintln;
use cortex_m_rt::{entry, exception, ExceptionFrame};

use embedded_hal::spi::{Mode, Phase, Polarity};
use embedded_sdmmc::{DirEntry, TimeSource, Timestamp};
use f3::hal::{
    gpio::{gpioa, gpioe, GpioExt},
    prelude::_stm32f30x_hal_flash_FlashExt,
    rcc::{Clocks, RccExt, APB2},
    spi::Spi,
    stm32f30x::{self, FLASH, GPIOA, RCC, SPI1},
    time::Hertz,
};
use heapless::String;
// use heapless::Vec;
// use panic_halt as _;
use panic_itm; // panic handler

// use state_mgmt;

use alloc_cortex_m::CortexMHeap;
use bitcoin::{
    secp256k1::{self, ffi::types::AlignedType, Secp256k1},
    util::bip32::ExtendedPrivKey,
    Address, Network, PrivateKey,
};

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024 * 40; // 40 KB .. the RAM size on f3discovery

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = stm32f30x::Peripherals::take().unwrap();

    // &mut rcc.ahbenr(|w| w.iopeen().set_bit())

    // let mut sm = state_mgmt::mm_state::MMState::new();
    // iprintln!(&mut cp.ITM.stim[0], "{:?}", sm.current_screen);
    // let did_update = sm.update_state(state_mgmt::mm_state_action::MMStateAction::Enter);

    // iprintln!(&mut cp.ITM.stim[0], "{:?}", did_update);
    // iprintln!(&mut cp.ITM.stim[0], "{:?}", sm.current_screen);
    //
    // iprintln!(&mut cp.ITM.stim[0], "heap size {}", HEAP_SIZE);

    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let size = Secp256k1::preallocate_size();
    // iprintln!(&mut cp.ITM.stim[0], "secp buf size {}", size * 16);

    // let mut buf_ful = vec![0; size];
    let altz = AlignedType::zeroed();
    let altxvec = vec![AlignedType::zeroed(); 1];
    let pre_size = (HEAP_SIZE - 100) / 16;
    let mut buf_ful2 = vec![AlignedType::zeroed(); pre_size];
    let secp = Secp256k1::preallocated_new(&mut buf_ful2).unwrap();

    // Load a private key
    let raw = "L1HKVVLHXiUhecWnwFYF6L3shkf1E12HUmuZTESvBXUdx3yqVP1D";
    let pkr: Result<PrivateKey, state_mgmt::bitcoin::util::key::Error> = PrivateKey::from_wif(raw);
    // let xprv: Result<ExtendedPrivKey, state_mgmt::bitcoin::util::bip32::Error> =
    // ExtendedPrivKey::new_master(Network::Bitcoin, &[1 as u8, 2 as u8]);
    // if pkr.is_err() {
    //     iprintln!(&mut cp.ITM.stim[0], "Is err");
    // }
    let pk = pkr.unwrap();
    // iprintln!(&mut cp.ITM.stim[0], "Seed WIF: {}", pk);

    // let mut buf_ful = vec![AlignedType::zeroed(); size];
    // let secp = Secp256k1::preallocated_new(&mut buf_ful).unwrap();

    // // Derive address
    let pubkey = pk.public_key(&secp);
    let address = Address::p2wpkh(&pubkey, Network::Bitcoin).unwrap();
    // iprintln!(&mut cp.ITM.stim[0], "Address: {}", address);

    // let mut rcc_2 = dp.RCC.constrain();
    // let mut flash_2 = dp.FLASH.constrain();

    // let mut flash = unsafe { &*FLASH::ptr() };
    // let mut rcc = unsafe { &*RCC::ptr() };

    // let clocks = rcc_2.cfgr.freeze(&mut flash_2.acr);

    // struct MyTimeSouce;
    // impl TimeSource for MyTimeSouce {
    //     fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
    //         Timestamp {
    //             year_since_1970: 0,
    //             zero_indexed_month: 0,
    //             zero_indexed_day: 0,
    //             hours: 0,
    //             minutes: 0,
    //             seconds: 0,
    //         }
    //     }
    // }

    // rcc.ahbenr.write(|w| w.iopaen().set_bit());

    // // dp.GPIOA.moder.write(|w| {
    // //     w.moder4().();
    // //     w.moder5().output();
    // //     w.moder6().output();
    // //     w.moder7().output()
    // // });

    // let mut gpioa = dp.GPIOA.split(&mut rcc_2.ahb);

    // // let spi1: SPI1 = SPI1::
    // // let mut apb2: APB2 = APB2 { _0: () };

    // let slave_select = gpioa
    //     .pa4
    //     .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    // let s_clock = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    // let s_miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    // let s_mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    // UNCOMMENT ALL THIS TO GET THE SD CARD BACK
    // let sd_card_spi = Spi::spi1(
    //     dp.SPI1,
    //     (s_clock, s_miso, s_mosi),
    //     Mode {
    //         polarity: Polarity::IdleLow,
    //         phase: Phase::CaptureOnFirstTransition,
    //     },
    //     Hertz(4_000_000),
    //     clocks,
    //     &mut rcc_2.apb2,
    // );

    // let sd_mmc_spi = embedded_sdmmc::SdMmcSpi::new(sd_card_spi, slave_select);

    // let mut cont = embedded_sdmmc::Controller::new(sd_mmc_spi, MyTimeSouce {});

    // dp.GPIOE.odr.write(|w| w.odr8().set_bit());

    // write_to.write_u8(100);

    // UNCOMMENT ALL THIS TO GET THE SD CARD BACK
    //
    // match cont.device().init() {
    //     Ok(_) => {
    //         // cont.device().spi().borrow_mut();
    //         // write!(uart, "OK!\nCard size...").unwrap();
    //         match cont.device().card_size_bytes() {
    //             Ok(size) => iprintln!(&mut cp.ITM.stim[0], "{}", size),
    //             Err(e) => iprintln!(&mut cp.ITM.stim[0], "Err here"),
    //         }

    //         // write!(uart, "Volume 0...").unwrap();
    //         iprintln!(&mut cp.ITM.stim[0], "1");

    //         // for i in 0..=12 {
    //         // let i = 0;
    //         match cont.get_volume(embedded_sdmmc::VolumeIdx(0)) {
    //             Ok(mut volume) => {
    //                 let root_dir = cont.open_root_dir(&volume).unwrap();
    //                 let mut index = 0;
    //                 // let mut files: Vec<DirEntry, 20> = Vec::new();
    //                 let res = cont.iterate_dir(&volume, &root_dir, |entry| {
    //                     iprintln!(&mut cp.ITM.stim[0], "1{:?}", entry.name);
    //                     iprintln!(&mut cp.ITM.stim[0], "2{:?}", entry.size);

    //                     // if let Some(pointer) = files.get_mut(index) {
    //                     //     *pointer = entry;
    //                     // }
    //                     // index += 1;
    //                 });
    //                 res.ok();

    //                 let mut buffer = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    //                 let mut file = cont
    //                     .open_file_in_dir(
    //                         &mut volume,
    //                         &root_dir,
    //                         "hello.txt",
    //                         embedded_sdmmc::Mode::ReadOnly,
    //                     )
    //                     .unwrap();
    //                 cont.read(&volume, &mut file, &mut buffer);
    //                 iprintln!(&mut cp.ITM.stim[0], "2{:?}", index);
    //                 // let mut file = cont
    //                 //     .open_file_in_dir(
    //                 //         &mut volume,
    //                 //         &root_dir,
    //                 //         "hello.txt",
    //                 //         embedded_sdmmc::Mode::ReadOnly,
    //                 //     )
    //                 //     .unwrap();
    //                 // let mut buffer = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    //                 // cont.read(&volume, &mut file, &mut buffer).unwrap();

    //                 // for item in buffer {
    //                 //     iprintln!(&mut cp.ITM.stim[0], "{}", item)
    //                 // }
    //             }
    //             Err(e) => {
    //                 iprintln!(&mut cp.ITM.stim[0], "1{:?}", e);
    //                 iprintln!(&mut cp.ITM.stim[0], "2{:?}", e);
    //                 iprintln!(&mut cp.ITM.stim[0], "3{:?}", e);
    //                 iprintln!(&mut cp.ITM.stim[0], "4{:?}", e)
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         iprintln!(&mut cp.ITM.stim[0], "1{:?}", e);
    //         iprintln!(&mut cp.ITM.stim[0], "2{:?}", e);
    //         iprintln!(&mut cp.ITM.stim[0], "3{:?}", e);
    //         iprintln!(&mut cp.ITM.stim[0], "4{:?}", e)
    //     }
    // }

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
    // if let Ok(mut hstdout) = hio::hstdout() {
    //     writeln!(hstdout, "{:#?}", ef).ok();
    // }

    loop {
        cortex_m::asm::bkpt();
        cortex_m::asm::nop();
    }
}

// #[panic_handler]
// #[link_section = ".text.asm_panic_handler"]
// fn panic(_: &core::panic::PanicInfo) -> ! {
//     loop {
//         cortex_m::asm::bkpt();
//     }
//     // __cortex_m_should_not_panic();
// }
