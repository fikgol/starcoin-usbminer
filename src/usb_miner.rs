use crate::constants::*;
use crate::read_until;
use anyhow::Result;
use serialport::{SerialPort, SerialPortSettings};
use std::io::BufReader;
use std::io::Write;
use crate::usb_miner::DeriveResponse::SolvedJob;
use crate::proto::{Message, DeriveResponse, State};
use std::time::Duration;

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

pub struct UsbMiner {
    serial_port: Box<dyn SerialPort>,
    config: Config,
    port_buf_reader: BufReader<Box<dyn SerialPort>>,
}

impl UsbMiner {
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

    pub fn read(&mut self) -> Result<DeriveResponse> {
        let mut raw_resp = vec![];
        read_until(&mut self.port_buf_reader, &PKT_ENDER, raw_resp.as_mut())?;
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
