pub(crate) const PKT_HEADER: [u8; 3] = [0xA5, 0x3C, 0x96];
pub(crate) const PKT_ENDER: [u8; 3] = [0x69, 0xC3, 0x5A];

pub(crate) const PV: u8 = 0x10;
pub(crate) const TYPE_SET_BOOT_MODE: u8 = 0xA3;
pub(crate) const TYPE_UPDATE_FW: u8 = 0xAA;
pub(crate) const TYPE_QUERY_INFO: u8 = 0xA4;
pub(crate) const TYPE_PRODUCT_TEST: u8 = 0xAB;
pub(crate) const TYPE_SET_LED: u8 = 0xA6;
pub(crate) const TYPE_SEND_OPCODE: u8 = 0xA7;
pub(crate) const TYPE_SEND_WORK: u8 = 0xA1;
pub(crate) const TYPE_SET_HWPARAMS: u8 = 0xA2;
pub(crate) const TYPE_REBOOT: u8 = 0xAC;
pub(crate) const TYPE_RECV_NONCE: u8 = 0x51;
pub(crate) const TYPE_RECV_STATE: u8 = 0x52;
pub(crate) const TYPE_RECV_BOOT_MODE: u8 = 0x53;
pub(crate) const TYPE_RECV_INFO: u8 = 0x54;
pub(crate) const TYPE_RECV_OP: u8 = 0x57;
pub(crate) const TYPE_RECV_FWSTATE: u8 = 0x5A;
pub(crate) const TYPE_RECV_TEST_RESULT: u8 = 0x5B;

pub(crate) const ALGO_VARITY: u32 = 4;
pub(crate) const TYPE_OFFSET: usize = 0;