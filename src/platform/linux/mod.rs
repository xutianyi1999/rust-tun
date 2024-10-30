//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

//! Linux specific functionality.

pub mod sys;

mod device;
pub use self::device::{Device, Queue};

use crate::configuration::Configuration as C;
use crate::error::*;

/// Linux-only interface configuration.
#[derive(Copy, Clone, Default, Debug)]
pub struct Configuration {
    pub(crate) packet_information: bool,

    /// Enable IFF_NAPI
    pub(crate) napi: bool,

    /// Enable IFF_VNET_HDR
    pub(crate) vnet_hdr: bool,
}

impl Configuration {
    /// Enable or disable packet information, when enabled the first 4 bytes of
    /// each packet is a header with flags and protocol type.
    pub fn packet_information(&mut self, value: bool) -> &mut Self {
        self.packet_information = value;
        self
    }


    /// Enable / Disable IFF_NAPI flag.
    pub fn napi(&mut self, value: bool) -> &mut Self {
        self.napi = value;
        self
    }

    /// Enable / Disable IFF_VNET_HDR flag.
    pub fn vnet_hdr(&mut self, value: bool) -> &mut Self {
        self.vnet_hdr = value;
        self
    }
}

/// Create a TUN device with the given name.
pub fn create(configuration: &C) -> Result<Device> {
    Device::new(configuration)
}
