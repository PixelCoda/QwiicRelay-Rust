# Qwiic Relay I2C library for Rust (WIP)

## Description

This library aims at controlling Qwiic Relays using I2C from Linux. It
primary target is ARM devices such as RaspberryPi or FriendlyARM's NanoPi Neo.
It should nonetheless work on other Linux distributions with access to an I2C
bus.

Currently I only have access to the Quad Solid State Relay for testing purposes. If you have issues with other Qwiic Relays please submit an issue or a pull request.

Roadmap:
* Map relay commands and addresses to structs (DONE)
* Ability to toggle all relays on/off (DONE)
* Ability to toggle individual relays on/off (WIP)
* Ability to read relay status (WIP)
* Ability to change relay hardware address (WIP)
* Ability to check firmware version (WIP)

## How to use library

Add the following line to your cargo.toml:
```
qwiic-relay-rs = { git = "https://github.com/PixelCoda/QwiicLCD-Rust.git", version = "*" }
```

Example:
```rust
extern crate qwiic_relay_rs;

use qwiic_relay_rs::*;
use std::thread;
use std::time::Duration;

fn main() {
    let _config = QwiicRelayConfig::default();

    let config = QwiicRelayConfig::default();
    let mut qwiic_relay = QwiicRelay::new(config, "/dev/i2c-1", 0x08).expect("Could not init device");

    println!("all off");
    qwiic_relay.set_all_relays_off();
    thread::sleep(Duration::from_secs(2));

    println!("all on");
    qwiic_relay.set_all_relays_on();
    thread::sleep(Duration::from_secs(2));

    println!("all off");
    qwiic_relay.set_all_relays_off();
    thread::sleep(Duration::from_secs(2));
}
```

## References

* https://github.com/sparkfun/Qwiic_Relay_Py/blob/main/qwiic_relay.py
* https://github.com/sparkfun/SparkFun_Qwiic_Relay_Arduino_Library/tree/master/src

## License

Released under Apache 2.0.
