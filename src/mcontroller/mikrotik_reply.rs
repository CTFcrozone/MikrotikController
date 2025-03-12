use mikrotik_rs::protocol::{CommandResponse, ReplyResponse};
use crate::mcontroller::{Error, Result};


// needs some cleanup and refactoring

pub struct MikrotikReply {
    pub response: CommandResponse
}


impl MikrotikReply {
    pub fn new(response: CommandResponse) -> Self {
        MikrotikReply {
            response
        }
    }

    pub fn check_error_response(&self) -> Result<()> {
        match &self.response {
            CommandResponse::Trap(resp) => Err(Error::TrapResponse { message: resp.message.to_string() }),
            CommandResponse::Fatal(resp) => Err(Error::FatalResponse {message: resp.to_string()}),
            _ => Ok(())

        }
    }

    pub fn check_done_response(&self) -> Result<()> {
        match &self.response {
            CommandResponse::Done(_) => Ok(()),
            _ => Err(Error::CommandCompletionFail)
        }
    }

    pub fn get_reply_response(&self) -> Result<ReplyResponse> {
        match &self.response {
            CommandResponse::Reply(resp) => Ok(resp.clone()),
            _ => Err(Error::NoReplyResponse)
        }
    }

}
