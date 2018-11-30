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
//! use drs_0x01::*;
//!
//! let servo = Servo::new(0x40);
//! let message = servo.set_speed(512, Rotation::Clockwise);
//! ```
//!
//! To reboot all the servomotors you can use this message :
//!
//! ```
//! # extern crate drs_0x01;
//! use drs_0x01::builder::MessageBuilder;
//! // 0xFE is the broadcast ID
//! let message = MessageBuilder::new().id(0xFE).reboot().build();
//! ```
//!
//! Here is how to enable torque for the servomotor labelled 35 :
//!
//! ```
//! # extern crate drs_0x01;
//! use drs_0x01::builder::MessageBuilder;
//! use drs_0x01::WritableRamAddr::TorqueControl;
//! let message = MessageBuilder::new_with_id(35).write_ram(TorqueControl(1)).build();
//! ```

#![no_std]
#![warn(missing_docs)]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate arrayvec;
extern crate try_from;

pub mod addr;
/// A module which implement the builder pattern to create advanced messages
pub mod builder;
mod message;
/// A module which contains a Finite State Machine to transform bytes read form the servomotor
/// into `[ACKPacket]s`
pub mod reader;
mod servo;

pub use addr::{ReadableEEPAddr, ReadableRamAddr, WritableEEPAddr, WritableRamAddr};
pub use message::{JogColor, JogMode, Rotation};
pub use servo::Servo;
