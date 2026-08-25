use crate::{constants::PAGE_SIZE, enums::page_type::PageType, errors::page_error::PageError};

pub const HEADER_SIZE: usize = 24;

const PAGE_ID_OFFSET: usize = 0; //Bytes 0-8 
const PAGE_TYPE_OFFSET: usize = 8; //Bytes 8-10
const SLOT_ARR_OFFSET: usize = 10; //Bytes 10-12
const REC_DATA_OFFSET: usize = 12; //Bytes 12-14
const DEAD_BYTES_OFFSET: usize = 14; //Bytes 14-16
const LSN_OFFSET: usize = 16; //Bytes 16-24

pub struct PageHeader {
    pub page_id: u64,             //8bytes
    pub page_type: PageType,      //2bytes
    pub slot_array_offset: u16,   //2bytes
    pub record_data_offset: u16,  //2bytes
    pub dead_bytes_counter: u16,  //2bytes
    pub log_sequence_number: u64, //8bytes
}

impl PageHeader {
    //read and parse the header directly from the start of a raw bytes buffer that we'll get from the buffer pool
    pub fn from_bytes(src: &[u8; PAGE_SIZE]) -> Result<Self, PageError> {
        Ok(Self {
            page_id: u64::from_be_bytes(src[PAGE_ID_OFFSET..PAGE_TYPE_OFFSET].try_into().unwrap()),
            page_type: PageType::try_from(u16::from_be_bytes(
                src[PAGE_TYPE_OFFSET..SLOT_ARR_OFFSET].try_into().unwrap(),
            ))?,
            slot_array_offset: u16::from_be_bytes(
                src[SLOT_ARR_OFFSET..REC_DATA_OFFSET].try_into().unwrap(),
            ),
            record_data_offset: u16::from_be_bytes(
                src[REC_DATA_OFFSET..DEAD_BYTES_OFFSET].try_into().unwrap(),
            ),
            dead_bytes_counter: u16::from_be_bytes(
                src[DEAD_BYTES_OFFSET..LSN_OFFSET].try_into().unwrap(),
            ),
            log_sequence_number: u64::from_be_bytes(
                src[LSN_OFFSET..HEADER_SIZE].try_into().unwrap(),
            ),
        })
    }

    //Serialize the header values back into the start of a raw byte buffer
    pub fn to_bytes(&self, dest: &mut [u8; PAGE_SIZE]) {
        dest[PAGE_ID_OFFSET..PAGE_TYPE_OFFSET].copy_from_slice(&self.page_id.to_be_bytes());
        dest[PAGE_TYPE_OFFSET..SLOT_ARR_OFFSET]
            .copy_from_slice(&(self.page_type as u16).to_be_bytes());
        dest[SLOT_ARR_OFFSET..REC_DATA_OFFSET]
            .copy_from_slice(&self.slot_array_offset.to_be_bytes());
        dest[REC_DATA_OFFSET..DEAD_BYTES_OFFSET]
            .copy_from_slice(&self.record_data_offset.to_be_bytes());
        dest[DEAD_BYTES_OFFSET..LSN_OFFSET].copy_from_slice(&self.dead_bytes_counter.to_be_bytes());
        dest[LSN_OFFSET..HEADER_SIZE].copy_from_slice(&self.log_sequence_number.to_be_bytes());
    }
}
