#[derive(Debug)]
pub enum PageError {
    InvalidPageType(u16),
    PageRetrievalError(u16),
}
