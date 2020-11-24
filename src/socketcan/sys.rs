use std::os::raw::{c_int, c_short};

const CAN_EFF_FLAG: u32 = 0x80000000;
const CAN_RTR_FLAG: u32 = 0x40000000;
const CAN_ERR_FLAG: u32 = 0x20000000;

const CAN_SFF_MASK: u32 = 0x7FF;
const CAN_EFF_MASK: u32 = 0x1FFFFFFF;
const CAN_ERR_MASK: u32 = 0x1FFFFFFF;

const CAN_SFF_ID_BITS: u32 = 11;
const CAN_EFF_ID_BITS: u32 = 29;

const CAN_MAX_DLC: usize = 8;
const CAN_MAX_DLEN: usize = 8;

pub const CAN_RAW: usize = 1;

pub const AF_CAN: c_int = 29;

#[repr(C)]
pub(crate) struct CanFrame {
    id: u32,
    dlc: u8,
    pad: u8,
    res0: u8,
    res1: u8,
    data: [u8; CAN_MAX_DLEN],
}


pub enum CanFrameError {
    IdTooLong,
    DataTooLong,
}

impl CanFrame {
    pub(crate) fn new_data(id: u32, ext_id: bool, data: &[u8]) -> Result<Self, CanFrameError> {
        let mut id = Self::validate_id(id, ext_id)?;

        if data.len() > CAN_MAX_DLEN {
            return Err(CanFrameError::DataTooLong);
        }

        let mut can_data = [0_u8; CAN_MAX_DLEN];
        can_data[0..data.len()].copy_from_slice(data);

        Ok(Self {
            id,
            dlc: data.len() as u8,
            pad: 0,
            res0: 0,
            res1: 0,
            data: can_data
        })
    }

    pub(crate) fn new_rtr(id: u32, ext_id: bool, dlc: u8) -> Result<Self, CanFrameError> {
        let mut id = Self::validate_id(id, ext_id)?;
        id |= CAN_RTR_FLAG;
        Ok(Self {
            id,
            dlc,
            pad: 0,
            res0: 0,
            res1: 0,
            data: [0_u8; CAN_MAX_DLEN],
        })
    }

    fn validate_id(id: u32, ext_id: bool) -> Result<u32, CanFrameError> {
        let mut id = id;
        if ext_id {
            if id > CAN_EFF_MASK {
                return Err(CanFrameError::IdTooLong);
            }
            id |= CAN_EFF_FLAG;
        } else {
            if id > CAN_SFF_MASK {
                return Err(CanFrameError::IdTooLong);
            }
        }
        Ok(id)
    }
}

#[repr(C)]
pub(crate) struct SocketAddr {
    pub(crate) _af_can: c_short,
    pub(crate) if_index: c_int, // address familiy,
    pub(crate) rx_id: u32,
    pub(crate) tx_id: u32,
}