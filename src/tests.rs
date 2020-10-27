#[cfg(test)]
mod tests {
    use super::*;
    use crate::usb_miner::{Config, UsbMiner};
    use crate::read_until;
    use std::io;

    #[test]
    fn test_read_until() {
        let mut buf = vec![];
        let n = read_until(&mut io::Cursor::new(b"abcdef"), b"cd", buf.as_mut()).unwrap();
        assert_eq!(4, n);
        assert_eq!(b"abcd".to_vec(), buf);

        let mut buf = vec![];
        let n = read_until(&mut io::Cursor::new(b"abdef"), b"cd", buf.as_mut()).unwrap();
        assert_eq!(5, n);
        assert_eq!(b"abdef".to_vec(), buf);

        let mut buf = vec![];
        let n = read_until(&mut io::Cursor::new(b"a"), b"cd", buf.as_mut()).unwrap();
        assert_eq!(1, n);
        assert_eq!(b"a".to_vec(), buf);
    }

    #[test]
    fn test_miner() {
        let path = "/dev/cu.usbmodem2065325550561";
        let mut miner = UsbMiner::open(path, Config::default()).expect("Must open serial port");
        miner.get_state();
        miner.read();
    }

    #[test]
    fn test_set_hw_msg() {
        let msg = UsbMiner::set_hw_params_msg(600, 750);
        // freq:600, voltage:750, varity:4
        let expect_msg: [u8; 22] = [0xa5, 0x3c, 0x96, 0xa2, 0x10, 0x10, 0x00, 0x00, 0x00, 0xa2, 0xee, 0x02, 0x58, 0x02, 0x04, 0x00, 0x00, 0x00, 0x50, 0x69, 0xc3, 0x5a];
        assert_eq!(msg, expect_msg);
    }

    #[test]
    fn test_get_state_msg() {
        let msg = UsbMiner::get_state_msg();
        let expect_msg: [u8; 13] = [0xa5, 0x3c, 0x96, 0xa2, 0x10, 0x07, 0x00, 0x00, 0x00, 0x52, 0x69, 0xc3, 0x5a];
        assert_eq!(expect_msg, msg.as_slice());
    }
}