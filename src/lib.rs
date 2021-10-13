// Copyright 2021 Caleb Mitchell Smith-Woolrich (PixelCoda)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate i2cdev;

use std::thread;
use std::time::Duration;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

#[derive(Copy, Clone)]
pub enum Addresses {
    SingleRelayDefault = 0x18,
    SingleRelayJumperClosed = 0x19,
    QuadRelayDefault = 0x6D,
    QuadRelayJumperClosed = 0x6C,
    DualSolidState = 0x0A,
    DualSolidStateJumperClosed = 0x0B,
    QuadSolidState = 0x08,
    QuadSolidStateJumperClosed = 0x09
}

#[derive(Copy, Clone)]
pub enum Commands {
    ToggleRelayOne = 0x01,
    ToggleRelayTwo = 0x02,
    ToggleRelayThree = 0x03,
    ToggleRelayFour = 0x04,
    RelayOneStaus = 0x05,
    RelayTwoStaus = 0x06,
    RelayThreeStaus = 0x07,
    RelayFourStaus = 0x08,
    TurnAllOff = 0x0A,
    TurnAllOn = 0x0B,
    ToggleAll = 0x0C
}

#[derive(Copy, Clone)]
pub enum RelayState {
    Off = 0x00,
    On = 0x01,
    SingleFirmwareVersion = 0x04,
    SingleStatusVersion = 0x05,
}

pub struct Config {
    relay_count: u8
}

impl Config {
    pub fn new(relay_count: u8) -> QwiicRelayConfig {
        Config {
            relay_count: 4,
        }
    }

    pub fn default() -> QwiicRelayConfig {
        QwiicRelayConfig::new(4)
    }
}

// QwiicRelay
pub struct QwiicRelay {
    dev: LinuxI2CDevice,
    config: QwiicRelayConfig,
}

type RelayResult = Result<(), LinuxI2CError>;

impl QwiicRelay {
    pub fn new(config: QwiicRelayConfig, bus: &str, i2c_addr: u16) -> Result<QwiicRelay, LinuxI2CError> {
        let dev = LinuxI2CDevice::new(bus, i2c_addr)?;
        Ok(QwiicRelay {
               dev,
               config,
           })
    }

    pub fn init(&mut self) -> RelayResult {

  
        // Wait for the QwiicRelay to set up
        thread::sleep(Duration::from_millis(200));

        Ok(())
    }



    pub fn set_all_relays_on(&mut self) -> RelayResult {
        self.write_byte(Command::TurnAllOn as u8)
    }

    pub fn set_all_relays_off(&mut self) -> RelayResult {
        self.write_byte(Command::TurnAllOff as u8)
    }

    pub fn write_byte(&mut self, command: u8) -> RelayResult {
        self.dev.smbus_write_byte(command)?;
        thread::sleep(Duration::new(0, 10_000));
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let _config = QwiicRelayConfig::default();

        let config = QwiicRelayConfig::default();
        let mut QwiicRelay = QwiicRelay::new(config, "/dev/i2c-1", 0x08).expect("Could not init device");
    

        thread::sleep(Duration::from_secs(1));
    }
}


