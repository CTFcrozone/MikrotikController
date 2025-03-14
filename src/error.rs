use derive_more::From;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // TBC
    #[from]
    Mikrotik(mikrotik_rs::error::DeviceError),

    #[from]
    MikrotikController(crate::mcontroller::Error),

    #[from]
    Clap(clap::error::Error),

    #[from]
    Io(std::io::Error),
}
// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
