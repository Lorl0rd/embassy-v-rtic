#![no_std]
#![no_main]

use core::cell::RefCell;

use defmt::*;
use embassy_executor::{task, Spawner};
use embassy_stm32::{gpio::{AnyPin, Level, Output, Speed}, Config};
use embassy_sync::blocking_mutex::CriticalSectionMutex;
use embassy_time::Timer;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

struct Shared<'d> {
    red: Output<'d, AnyPin>,
    green: Output<'d, AnyPin>,
    yellow: Output<'d, AnyPin>,
}

type MutRefShared = CriticalSectionMutex<RefCell<Shared<'static>>>;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    info!("Hello, world!");

    let red = Output::new(AnyPin::from(p.PB14), Level::High, Speed::Low);
    let green = Output::new(AnyPin::from(p.PB0), Level::High, Speed::Low);
    let yellow = Output::new(AnyPin::from(p.PE1), Level::High, Speed::Low);

    static SHARED: StaticCell<CriticalSectionMutex<RefCell<Shared>>> = StaticCell::new();
    let shared = &*SHARED.init(CriticalSectionMutex::new(RefCell::new( Shared { red, green, yellow })));
    // let shared = CriticalSectionMutex::new(RefCell::new( Shared { red, green, yellow }));

    unwrap!(spawner.spawn(blink_red(&shared)));
    unwrap!(spawner.spawn(blink_green(&shared)));
    unwrap!(spawner.spawn(blink_yellow(&shared)));
}

#[task]
async fn blink_red(shared: &'static MutRefShared) -> ! {
    loop {
        shared.lock(|cell| {
            cell.borrow_mut().red.toggle();
        });
        Timer::after_millis(250).await;
    }
}

#[task]
async fn blink_green(shared: &'static MutRefShared) -> ! {
    loop {
        shared.lock(|cell| {
            cell.borrow_mut().green.toggle();
        });
        Timer::after_millis(500).await;
    }
}

#[task]
async fn blink_yellow(shared: &'static MutRefShared) -> ! {
    loop {
        shared.lock(|cell| {
            cell.borrow_mut().yellow.toggle();
        });
        Timer::after_millis(1000).await;
    }
}
