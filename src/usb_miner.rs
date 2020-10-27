use anyhow::Result;
use serialport::{SerialPort, SerialPortSettings};
use crate::constants::*;
use crate::{proto_msg, read_until};
use std::io::{Write, Cursor};
use std::borrow::{BorrowMut, Borrow};
use std::time::{Duration, SystemTime};
use byteorder::{WriteBytesExt, LittleEndian, ReadBytesExt};
use std::convert;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use std::ops::IndexMut;

pub struct UsbMiner {
    serial_port: Box<dyn SerialPort>,
    config: Config,
    port_buf_reader: BufReader<Box<dyn SerialPort>>,

}

impl UsbMiner {
    pub fn get_state(&mut self) -> Result<()> {
        let msg = Self::get_state_msg();
        let _ = self.serial_port.write(&msg)?;
        Ok(())
    }

    pub fn get_state_msg() -> Vec<u8> {
        proto_msg!(
            PKT_HEADER,
            [TYPE_SET_HWPARAMS],
            [PV],
            [0x7, 0x0, 0x0, 0x0],
            [0x52],
            PKT_ENDER)
    }

    pub fn open(path: &str, config: Config) -> Result<Self> {
        let mut setting = SerialPortSettings::default();
        setting.baud_rate = 115200;
        setting.timeout = Duration::from_secs(3);
        let serial_port = serialport::open_with_settings(path, &setting)?;
        let port_buf_reader = BufReader::new(serial_port.try_clone().unwrap());
        Ok(Self {
            serial_port,
            config,
            port_buf_reader,
        })
    }

    pub fn read(&mut self) -> Result<()> {
        let mut data = vec![];
        read_until(&mut self.port_buf_reader, &PKT_ENDER, data.as_mut());
        let location = data.windows(PKT_HEADER.len())
            .position(|w| w == PKT_HEADER)
            .ok_or(anyhow::anyhow!("Receive Invalid PKT"))?;
        let type_location = location + PKT_HEADER.len() + TYPE_OFFSET;
        let data_type = &data[type_location];
        match data_type {
            &TYPE_RECV_STATE => {
                let state = State::new(&data)?;
                println!("{:?}",&state);
            }
            _ => {}
        }
        Ok(())
    }

    pub fn set_hw_params(&mut self) -> Result<()> {
        let msg = UsbMiner::set_hw_params_msg(self.config.target_freq, self.config.target_voltage);
        let _ = self.serial_port.write(&msg)?;
        Ok(())
    }

    pub fn set_hw_params_msg(freq: u16, voltage: u16) -> Vec<u8> {
        let mut freq_b = vec![];
        let mut voltage_b = vec![];
        let mut varity_b = vec![];
        freq_b.write_u16::<LittleEndian>(freq).unwrap();
        voltage_b.write_u16::<LittleEndian>(voltage).unwrap();
        varity_b.write_u32::<LittleEndian>(ALGO_VARITY);
        let pktlen: [u8; 4] = [0x10, 0x00, 0x00, 0x00];
        let flag: [u8; 1] = [0xA2];
        let target_temp: [u8; 1] = [80];

        proto_msg!(PKT_HEADER,
                        [TYPE_SET_HWPARAMS],
                        [PV],
                        pktlen,
                        flag,
                        voltage_b,
                        freq_b,
                        varity_b,
                        target_temp,
                        PKT_ENDER)
    }
}

pub struct Config {
    pub target_freq: u16,
    pub target_voltage: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target_freq: 600,
            target_voltage: 750,
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub chips: u8,
    pub cores: u8,
    pub goodcores: u8,
    pub scanbits: u8,
    pub scantime: u16,
    pub voltage: u16,
    pub freq: u16,
    pub varity: u32,
    pub temp: u8,
    pub hwreboot: u8,
    pub tempwarn: u8,
    pub latest_updated: Duration,
}

impl State {
    fn new(raw_data: &[u8]) -> Result<Self> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("System time is before the UNIX_EPOCH");
        let mut data = Cursor::new(&raw_data[13..]);
        Ok(Self {
            chips: raw_data[9],
            cores: raw_data[10],
            goodcores: raw_data[11],
            scanbits: raw_data[12],
            scantime: data.read_u16::<LittleEndian>()?,
            voltage: data.read_u16::<LittleEndian>()?,
            freq: data.read_u16::<LittleEndian>()?,
            varity: data.read_u32::<LittleEndian>()?,
            temp: raw_data[23],
            hwreboot: raw_data[24],
            tempwarn: raw_data[25],
            latest_updated: now,
        })
    }
}