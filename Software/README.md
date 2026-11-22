# Radio Puck firmware

Rust firmware for the Radio Puck board (STM32F407VGT6), using
[Embassy](https://embassy.dev). The pin map is at the top of
[`src/main.rs`](src/main.rs).

## Setup

```sh
rustup target add thumbv7em-none-eabihf
# probe-rs, for flashing and logs (also needs udev rules, see probe.rs docs)
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

Connect a debug probe to J1 and set **SW1-1 ON** (boot from flash).

## Run

```sh
cargo run            # build, flash, and stream logs
cargo run --release
```

The current firmware runs an LED chaser across LED1..LED8 and logs `tick`.

## Hardware notes

* SW1-1 OFF (BOOT0 high, R32 pull-up) boots the ROM bootloader instead of
  the firmware.
* MDIO (PA2) has no pull-up; the DP83822 expects one (~1.5-2.2 kΩ). Until
  then, enable the STM32's internal pull-up on PA2 when bringing up Ethernet.
* The DP83822 gets its 25 MHz clock from MCO1 (PA8), so MCO1 must be set to
  output HSE before the PHY is released from reset (PC0).
* The radio TCXOs are 30 MHz; use TCXO mode in POWER_UP / the WDS config.
