use crate::utils::get_env;
use std::sync::OnceLock;

pub fn mikrotik_config() -> &'static MikrotikConfig {
    static INSTANCE: OnceLock<MikrotikConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        MikrotikConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct MikrotikConfig {
    pub MIKROTIK_ADDRESS: String,
    pub MIKROTIK_USERNAME: String,
    pub MIKROTIK_PWD: String,
}

impl MikrotikConfig {
    fn load_from_env() -> crate::utils::Result<MikrotikConfig> {
        Ok(MikrotikConfig {
            MIKROTIK_ADDRESS: get_env("MIKROTIK_ADDRESS")?,
            MIKROTIK_USERNAME: get_env("MIKROTIK_USERNAME")?,
            MIKROTIK_PWD: get_env("MIKROTIK_PWD")?,
        })
    }
}
