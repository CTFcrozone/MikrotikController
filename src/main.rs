use config::mikrotik_config;
pub use error::{Error, Result};
use mikrotik_rs::MikrotikDevice;
use tracing_subscriber::EnvFilter;

mod config;
mod entities;
mod error;
mod mcontroller;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {

    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    
    
    let dev = MikrotikDevice::connect(
        &mikrotik_config().MIKROTIK_ADDRESS,
        &mikrotik_config().MIKROTIK_USERNAME,
        Some(&mikrotik_config().MIKROTIK_PWD),
    )
    .await?;

    let controller = mcontroller::MikrotikController::from_mikrotik_device(dev);

    // controller.interface_list().await?;
    // controller.ip_address_add("192.168.1.34", "ether2").await?;
    controller.active_user_list().await?;

    Ok(())
}
