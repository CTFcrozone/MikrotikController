use crate::mcontroller::{Error, Result};
use mikrotik_rs::command;
use crate::mcontroller::MikrotikReply;
use tracing::debug;

pub struct MikrotikController {
    pub mikrotik_device: mikrotik_rs::MikrotikDevice,
}

impl MikrotikController {
    pub fn from_mikrotik_device(mikrotik_device: mikrotik_rs::MikrotikDevice) -> Self {
        MikrotikController { mikrotik_device }
    }
}

impl MikrotikController {
    pub async fn active_user_list(&self)->Result<()>{
        let cmd = command!("/user/active/print");

        let mut res = self.mikrotik_device.send_command(cmd).await;

        let reply = res.recv().await;

        let reply = reply.ok_or(Error::NoReply)?;

        let mikrotik_reply = MikrotikReply::new(reply?);
        mikrotik_reply.check_error_response()?;
        let reply_response = mikrotik_reply.get_reply_response()?;
        println!("{:#?}", reply_response.attributes);
        Ok(())

    }

    pub async fn interface_list(&self) -> Result<()> {
        println!("->> Interface list");
        let cmd = command!("/interface/print");

        let mut res = self.mikrotik_device.send_command(cmd).await;

        let reply = res.recv().await;

        let reply = reply.ok_or(Error::NoReply)?;

        let mikrotik_reply = MikrotikReply::new(reply?);
        mikrotik_reply.check_error_response()?;
        let reply_response = mikrotik_reply.get_reply_response()?;
        println!("{:#?}", reply_response.attributes);
        Ok(())
    }

    pub async fn ip_address_add(&self, addr: &str, iface: &str) -> Result<()> {
        let cmd = command!("/ip/address/add", address=addr, interface=iface);
        let mut res = self.mikrotik_device.send_command(cmd).await;

        let reply = res.recv().await;

        let reply = reply.ok_or(Error::NoReply)?;

        let mikrotik_reply = MikrotikReply::new(reply?);

        mikrotik_reply.check_error_response()?;
        mikrotik_reply.check_done_response()?;

        println!("->> IP address added successfully");
        
        Ok(())
    }

    // pub async fn check_update(&self) {
    //     // let cmd = command!("/system/package/update/install")
    //     todo!()
    // }

    pub async fn reboot(&self) -> Result<()> {
        let cmd = command!("/system/reboot");

        let mut res = self.mikrotik_device.send_command(cmd).await;

        let reply = res.recv().await;

        let reply = reply.ok_or(Error::NoReply)?;

        let mikrotik_reply = MikrotikReply::new(reply?);

        mikrotik_reply.check_error_response()?;
        mikrotik_reply.check_done_response()?;
        println!("->> Reboot executed successfully");

        Ok(())
    }
}
