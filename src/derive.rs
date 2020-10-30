use crate::constants::*;
use crate::derive::DeriveResponse::SolvedJob;
use crate::proto::{DeriveResponse, Message, State};
use crate::read_until;
use anyhow::Result;
use serialport::{SerialPort, SerialPortSettings};
use std::io::BufReader;
use std::io::Write;
use std::time::Duration;

pub struct Config {
    pub target_freq: u16,
    pub target_voltage: u16,
    pub read_timeout: Duration,
    baud_rate: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target_freq: 600,
            target_voltage: 750,
            read_timeout: Duration::from_secs(3),
            baud_rate: 115200,
        }
    }
}

pub struct UsbDerive {
    serial_port: Box<dyn SerialPort>,
    config: Config,
}

impl UsbDerive {
    pub fn open(path: &str, config: Config) -> Result<Self> {
        let mut setting = SerialPortSettings::default();
        setting.baud_rate = config.baud_rate;
        setting.timeout = config.read_timeout;
        let serial_port = serialport::open_with_settings(path, &setting)?;
        Ok(Self {
            serial_port,
            config,
        })
    }

    pub fn read(&mut self) -> Result<DeriveResponse> {
        let mut raw_resp = vec![];
        let mut port_buf_reader = BufReader::new(&mut self.serial_port);
        read_until(&mut port_buf_reader, &PKT_ENDER, raw_resp.as_mut())?;
        DeriveResponse::new(raw_resp)
    }

    pub fn get_state(&mut self) -> Result<()> {
        let msg = Message::get_state_msg();
        let _ = self.serial_port.write(&msg)?;
        Ok(())
    }

    pub fn set_hw_params(&mut self) -> Result<()> {
        let msg = Message::set_hw_params_msg(self.config.target_freq, self.config.target_voltage);
        let _ = self.serial_port.write(&msg)?;
        Ok(())
    }
    pub fn set_job(&mut self, job_id: u8, target: u32, data: [u8; 76]) -> Result<()> {
        let msg = Message::write_job_msg(job_id, target, data);
        let _ = self.serial_port.write(&msg)?;
        Ok(())
    }

    pub fn set_opcode(&mut self) -> Result<()> {
        let msg = Message::opcode_msg();
        let _ = self.serial_port.write(&msg)?;
        Ok(())
    }
}
