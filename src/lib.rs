//! This crate provides basic functionnality to communicate with Herkulex DRS (both 0101 and
//! 0201, other model are not supported even if they might partially work) servomotors.
//! It is heavily based on the documentation published by Dongbu Robot which is available
//! [`here`](http://www.sgbotic.com/products/datasheets/robotics/herkulexeng.pdf).
//!
//! # Examples
//!
//! To set a servo to a position, you can use this message :
//!
//! ```
//! # extern crate drs_0x01;
//! use drs_0x01::prelude::*;
//!
//! let servo = Servo::new(0x40);
//! let message = servo.set_speed(512);
//! ```
//!
//! To reboot all the servomotors you can use this message :
//!
//! ```
//! # extern crate drs_0x01;
//! use drs_0x01::advanced::MessageBuilder;
//! // 0xFE is the broadcast ID
//! let message = MessageBuilder::new().id(0xFE).reboot().build();
//! ```
//!
//! Here is how to enable torque for the servomotor labelled 35 :
//!
//! ```
//! # extern crate drs_0x01;
//! use drs_0x01::advanced::MessageBuilder;
//! use drs_0x01::addr::WritableRamAddr::TorqueControl;
//! let message = MessageBuilder::new_with_id(35).write_ram(TorqueControl(1)).build();
//! ```

#![no_std]
//#![deny(missing_docs)]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate arrayvec;
extern crate try_from;

pub mod addr;
mod builder;
mod message;
pub mod reader;
mod servo;

/// Advanced data types for experimented users knowing the datasheet.
pub mod advanced {
    pub use builder::{
        HerkulexMessage, MessageBuilder, MessageBuilderCmd, MessageBuilderError,
        MessageBuilderPositionIJOG, MessageBuilderPositionSJOG, MessageBuilderSpecial,
    };
}

/// Easy to use functions for a quickstart.
pub mod prelude {
    pub use message::{JogColor, JogMode};
    pub use servo::Servo;
}
