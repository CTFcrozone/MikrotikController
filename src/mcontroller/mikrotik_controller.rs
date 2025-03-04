use crate::mcontroller::{Error, Result};
use mikrotik_rs::command;

pub struct MikrotikController {
    pub mikrotik_device: mikrotik_rs::MikrotikDevice,
}

impl MikrotikController {
    pub fn from_mikrotik_device(mikrotik_device: mikrotik_rs::MikrotikDevice) -> Self {
        MikrotikController { mikrotik_device }
    }
}

impl MikrotikController {
    pub async fn active_user_list(&self) {
        // let cmd = command!("/user/active/print")
        todo!()
    }

    pub async fn interface_list(&self) {
        // let cmd = command!("/interface/print")
        todo!()
    }

    pub async fn ip_address_add(&self, addr: &str, iface: &str) {
        // let cmd = command!("/ip/address/add", address=addr, interface=iface)

        todo!()
    }

    pub async fn check_update(&self) {
        // let cmd = command!("/system/package/update/install")
        todo!()
    }

    pub async fn reboot(&self) {
        // let cmd = command!("/system/reboot")

        todo!()
    }
}
