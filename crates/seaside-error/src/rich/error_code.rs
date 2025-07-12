pub type ErrorCode = u16;

pub trait ToErrorCode {
    fn code(&self) -> ErrorCode;
}
