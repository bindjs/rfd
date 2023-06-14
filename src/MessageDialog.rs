use super::FileHandle::*;
use super::MessageButtons::*;
use super::MessageLevel::*;
use napi::bindgen_prelude::*;
use napi::*;
use rfd;
use std::path::PathBuf;

#[napi]
#[repr(transparent)]
pub struct MessageDialog(pub(super) Option<rfd::MessageDialog>);
#[napi]
impl MessageDialog {
    #[napi(constructor)]
    pub fn new() -> Self {
        return Self(Some(rfd::MessageDialog::new()));
    }

    #[napi]
    pub fn set_level(&mut self, level: MessageLevel) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_level(level.to_rfd_t());
        return Ok(Self(Some(x)));
    }

    #[napi]
    pub fn set_title(&mut self, title: String) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_title(&title);
        return Ok(Self(Some(x)));
    }

    #[napi]
    pub fn set_description(&mut self, description: String) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_description(&description);
        return Ok(Self(Some(x)));
    }

    #[napi]
    pub fn set_buttons(&mut self, buttons: MessageButtons) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_buttons(buttons.to_rfd_t());
        return Ok(Self(Some(x)));
    }

    #[napi]
    pub fn show(&mut self) -> Result<bool> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let y = x.show();
        return Ok(y);
    }
}