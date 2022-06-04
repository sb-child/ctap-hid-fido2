mod client_pin;
mod client_pin_command;
mod client_pin_response;
use super::FidoKeyHid;
use crate::ctaphid;
use anyhow::Result;
use client_pin_command::SubCommand as PinCmd;
pub use client_pin_command::*;
pub use client_pin_response::*;

impl FidoKeyHid {
    /// Get PIN retry count
    pub fn get_pin_retries(&self) -> Result<i32> {
        let cid = ctaphid::ctaphid_init(self)?;

        let send_payload = client_pin_command::create_payload(PinCmd::GetRetries)?;

        let response_cbor = ctaphid::ctaphid_cbor(self, &cid, &send_payload)?;

        let pin = client_pin_response::parse_cbor_client_pin_get_retries(&response_cbor)?;

        Ok(pin.retries)
    }

    /// Get UV retry count
    pub fn get_uv_retries(&self) -> Result<i32> {
        let cid = ctaphid::ctaphid_init(self)?;

        let send_payload = client_pin_command::create_payload(PinCmd::GetUVRetries)?;

        let response_cbor = ctaphid::ctaphid_cbor(self, &cid, &send_payload)?;

        let pin = client_pin_response::parse_cbor_client_pin_get_retries(&response_cbor)?;

        Ok(pin.uv_retries)
    }

    /// Set New PIN
    pub fn set_new_pin(&self, pin: &str) -> Result<()> {
        let cid = ctaphid::ctaphid_init(self)?;
        self.set_pin(&cid, pin)?;
        Ok(())
    }

    /// Change PIN
    pub fn change_pin(&self, current_pin: &str, new_pin: &str) -> Result<()> {
        let cid = ctaphid::ctaphid_init(self)?;
        client_pin::change_pin(self, &cid, current_pin, new_pin)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SubCommand as PinCmd;
    use super::*;
    use crate::ctaphid;
    use crate::fidokey::FidoKeyHid;
    use crate::Cfg;
    use crate::HidParam;

    #[test]
    fn test_client_pin_get_keyagreement() {
        let hid_params = HidParam::get();
        let device = FidoKeyHid::new(&hid_params, &Cfg::init()).unwrap();
        let cid = ctaphid::ctaphid_init(&device).unwrap();

        let send_payload = create_payload(PinCmd::GetKeyAgreement).unwrap();
        let response_cbor = ctaphid::ctaphid_cbor(&device, &cid, &send_payload).unwrap();

        let key_agreement =
            client_pin_response::parse_cbor_client_pin_get_keyagreement(&response_cbor).unwrap();
        println!("authenticatorClientPIN (0x06) - getKeyAgreement");
        println!("{}", key_agreement);

        assert!(true);
    }
}
