// #![deny(warnings)]
// #![deny(unsafe_code)]
#![no_std]
#![no_main]

use rtic_monotonics::systick::prelude::*;
use {defmt_rtt as _, panic_probe as _};

systick_monotonic!(Mono, 1000);

#[rtic::app(device = stm32h7xx_hal::stm32, peripherals = true, dispatchers = [EXTI1])]
mod app {
    use defmt::info;
    use stm32h7xx_hal::gpio::gpiob::{PB0, PB14};
    use stm32h7xx_hal::gpio::gpioe::PE1;
    use stm32h7xx_hal::gpio::Output;
    use stm32h7xx_hal::prelude::*;
    use rtic_monotonics::fugit::ExtU64;

    use super::*;

    #[shared]
    struct SharedResources {
        red: PB14<Output>,
        yellow: PE1<Output>,
        green: PB0<Output>,
    }

    #[local]
    struct LocalResources {}

    #[init]
    fn init(cx: init::Context) -> (SharedResources, LocalResources) {
        let pwr = cx.device.PWR.constrain();
        let pwrcfg = pwr.freeze();

        // RCC
        let rcc = cx.device.RCC.constrain();
        let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &cx.device.SYSCFG);

        // Initialize the systick interrupt & obtain the token to prove that we did
        Mono::start(cx.core.SYST, ccdr.clocks.sysclk().raw());

        // GPIO
        let gpiob = cx.device.GPIOB.split(ccdr.peripheral.GPIOB);
        let gpioe = cx.device.GPIOE.split(ccdr.peripheral.GPIOE);

        // LEDs
        let red = gpiob.pb14.into_push_pull_output();
        let yellow = gpioe.pe1.into_push_pull_output();
        let green = gpiob.pb0.into_push_pull_output();

        info!("Hello, world!");
        
        // Start tasks
        {
            blink_red::spawn().unwrap();
            blink_yellow::spawn().unwrap();
            blink_green::spawn().unwrap();
        }

        (SharedResources { red, yellow, green }, LocalResources {})
    }

    #[task(shared = [red])]
    async fn blink_red(mut cx: blink_red::Context) {
        loop {
            cx.shared.red.lock(|led| {
                led.toggle();
            });
            Mono::delay(ExtU64::millis(250)).await;            
        }
    }

    #[task(shared = [yellow])]
    async fn blink_yellow(mut cx: blink_yellow::Context) {
        loop {
            cx.shared.yellow.lock(|led| {
                led.toggle();
            });
            Mono::delay(ExtU64::millis(500)).await;            
        }
    }

    #[task(shared = [green])]
    async fn blink_green(mut cx: blink_green::Context) {
        loop {
            cx.shared.green.lock(|led| {
                led.toggle();
            });
            Mono::delay(ExtU64::millis(1000)).await;            
        }
    }
}
