use crate::errors::page_error::PageError;

#[derive(Clone, Copy)]
pub enum PageType {
    HeaderPage = 0x00,
    DataPage = 0x01,
    Freelist = 0x02,
}

impl TryFrom<u16> for PageType {
    type Error = PageError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::HeaderPage),
            0x01 => Ok(Self::DataPage),
            0x02 => Ok(Self::Freelist),
            _ => Err(PageError::InvalidPageType(value)),
        }
    }
}
