pub mod constants;
pub mod usb_miner;
mod tests;
#[macro_use]
extern crate lazy_static;

use std::io::{BufReader, BufRead};
use serialport::SerialPort;
use std::io;

#[macro_export]
macro_rules! proto_msg {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::<u8>::new();
            $(
                temp_vec.extend_from_slice($x.clone().as_ref());
            )*
            temp_vec
        }
    };
}

pub fn read_until(buf_reader: &mut BufRead, delim: &[u8], buf: &mut Vec<u8>) -> io::Result<usize> {
    let mut total_n = 0;
    loop {
        let mut tmp_buf = vec![];
        let n = buf_reader.read_until(delim[delim.len() - 1], tmp_buf.as_mut())?;
        total_n += n;
        buf.extend_from_slice(tmp_buf.as_slice());
        if n <= delim.len() {
            break;
        }
        if &tmp_buf[n - delim.len()..] == delim {
            break;
        }
    }
    Ok(total_n)
}
