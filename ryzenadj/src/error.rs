use ryzenadj_sys as sys;
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    #[error("RyzenAdj initialization failed")]
    InitializationFailed,
    #[error("CPU family is unsupported")]
    UnsupportedFamily,
    #[error("RyzenAdj returned invalid CPU family {0}")]
    InvalidFamily(i32),
    #[error("SMU request timed out")]
    SmuTimeout,
    #[error("SMU command is unsupported")]
    SmuUnsupported,
    #[error("SMU command was rejected")]
    SmuRejected,
    #[error("physical memory access failed")]
    MemoryAccess,
    #[error("RyzenAdj returned unknown status {0}")]
    UnknownStatus(i32),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub(crate) fn from_status(status: i32) -> Result<()> {
        match status {
            0 => Ok(()),
            sys::ADJ_ERR_FAM_UNSUPPORTED => Err(Error::UnsupportedFamily),
            sys::ADJ_ERR_SMU_TIMEOUT => Err(Error::SmuTimeout),
            sys::ADJ_ERR_SMU_UNSUPPORTED => Err(Error::SmuUnsupported),
            sys::ADJ_ERR_SMU_REJECTED => Err(Error::SmuRejected),
            sys::ADJ_ERR_MEMORY_ACCESS => Err(Error::MemoryAccess),
            code => Err(Error::UnknownStatus(code)),
        }
    }
}
