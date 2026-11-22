//! Radio Puck firmware (STM32F407VGT6).
//!
//! Pin map, from the schematic in `../Hardware`:
//!
//! | Function                 | Pins                                                         |
//! |--------------------------|--------------------------------------------------------------|
//! | LED1..LED8               | PD2..PD9, active high                                        |
//! | HSE 25 MHz TCXO (XTAL1)  | PH0 / OSC_IN (bypass mode)                                   |
//! | LSE 32.768 kHz osc (U2)  | PC14 / OSC32_IN (bypass mode)                                |
//! | Radio 1, Si4468 (U4)     | SPI1: SCK PA5, MISO PB4, MOSI PB5; NSEL PA15, SDN PD0, nIRQ PC7 |
//! | Radio 2, Si4468 (U5)     | SPI3: SCK PC10, MISO PC11, MOSI PC12; NSEL PA4, SDN PD1, nIRQ PC8 |
//! | Ethernet PHY, DP83822    | MII: MDC PC1, MDIO PA2, TX_CLK PC3, TX_EN PB11,              |
//! | (U6)                     | TXD0..3 PB12 PB13 PC2 PB8, RX_CLK PA1, RX_DV PA7, RX_ER PB10, |
//! |                          | RXD0..3 PC4 PC5 PB0 PB1, CRS PA0, COL PA3,                   |
//! |                          | nRESET PC0, INT/PWDN PC6, 25 MHz ref clock from MCO1 (PA8)   |
//! | USB-C (J2), OTG FS       | DM PA11, DP PA12 (no VBUS sensing)                           |
//! | SWD (J1)                 | SWDIO PA13, SWCLK PA14, SWO PB3, NRST                        |
//! | SW1-1 / SW1-2            | BOOT0 / BOOT1 (PB2); ON = low                                |
//!
//! Both radios have their own 30 MHz TCXO (Y1, Y2).

#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::time::Hertz;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(clock_config());
    info!("Radio Puck up");

    let mut leds = [
        Output::new(p.PD2, Level::Low, Speed::Low),
        Output::new(p.PD3, Level::Low, Speed::Low),
        Output::new(p.PD4, Level::Low, Speed::Low),
        Output::new(p.PD5, Level::Low, Speed::Low),
        Output::new(p.PD6, Level::Low, Speed::Low),
        Output::new(p.PD7, Level::Low, Speed::Low),
        Output::new(p.PD8, Level::Low, Speed::Low),
        Output::new(p.PD9, Level::Low, Speed::Low),
    ];

    // LED chaser: shows the clocks, timer and every LED are working.
    let mut i = 0;
    loop {
        leds[i].set_low();
        i = (i + 1) % leds.len();
        leds[i].set_high();
        if i == 0 {
            info!("tick");
        }
        Timer::after_millis(100).await;
    }
}

/// 25 MHz HSE -> PLL -> 168 MHz SYSCLK, with 48 MHz on PLLQ for USB/RNG.
/// APB1 = 42 MHz, APB2 = 84 MHz.
///
/// If nothing is ever logged, check XTAL1: init waits for the HSE to start.
fn clock_config() -> embassy_stm32::Config {
    use embassy_stm32::rcc::*;

    let mut config = embassy_stm32::Config::default();
    config.rcc.hse = Some(Hse {
        freq: Hertz(25_000_000),
        mode: HseMode::Bypass, // driven by an oscillator, not a crystal
    });
    config.rcc.pll_src = PllSource::HSE;
    config.rcc.pll = Some(Pll {
        prediv: PllPreDiv::DIV25,
        mul: PllMul::MUL336,
        divp: Some(PllPDiv::DIV2),
        divq: Some(PllQDiv::DIV7),
        divr: None,
    });
    config.rcc.sys = Sysclk::PLL1_P;
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV4;
    config.rcc.apb2_pre = APBPrescaler::DIV2;
    config
}
