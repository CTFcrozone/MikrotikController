use config::mikrotik_config;
pub use error::{Error, Result};
use mikrotik_rs::MikrotikDevice;

mod config;
mod entities;
mod error;
mod mcontroller;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let dev = MikrotikDevice::connect(
        &mikrotik_config().MIKROTIK_ADDRESS,
        &mikrotik_config().MIKROTIK_USERNAME,
        Some(&mikrotik_config().MIKROTIK_PWD),
    )
    .await?;

    Ok(())
}
