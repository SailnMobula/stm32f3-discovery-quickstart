#![no_std]
#![no_main]

use cortex_m::Peripherals as CorePeripherals;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3_discovery::{
    leds::{Direction, Leds},
    stm32f3xx_hal::{
        delay::Delay, gpio::GpioExt, pac::Peripherals as PacPeriphs,
        prelude::{_stm32f3xx_hal_flash_FlashExt, _embedded_hal_blocking_delay_DelayMs}, rcc::RccExt,
    },
    switch_hal::ToggleableOutputSwitch,
};

#[entry]
fn main() -> ! {
    let core_peripherals = CorePeripherals::take().unwrap();
    let device_peripherals = PacPeriphs::take().unwrap();

    let mut reset_and_clock_control = device_peripherals.RCC.constrain();
    let mut flash = device_peripherals.FLASH.constrain();

    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_peripherals.SYST, clocks);

    let mut gpioe = device_peripherals.GPIOE.split(&mut reset_and_clock_control.ahb);
    let mut leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );

    let toggle_time_ms: u32 = 1000;

    loop {
        leds.for_direction(Direction::North).toggle().ok();
        delay.delay_ms(toggle_time_ms);
    }
}
