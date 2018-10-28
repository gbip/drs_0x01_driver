[![Latest version](https://img.shields.io/crates/v/drs-0x01.svg)](https://crates.io/crates/drs-0x01)
[![Documentation](https://docs.rs/drs-0x01/badge.svg)](https://docs.rs/drs-0x01)
[![dependency status](https://deps.rs/repo/github/gbip/drs_0x01_driver/status.svg)](https://deps.rs/repo/github/gbip/drs_0x01_driver)

 # [Documentation](https://docs.rs/drs-0x01/0.1.6/drs_0x01/)
 
 This crate provides basic functionnality to communicate with Herkulex DRS (both 0101 and 0201)
 servomotors.
 It is heavily based on the documentation published by Dongbu Robot which is available
 [`here`](http://www.sgbotic.com/products/datasheets/robotics/herkulexeng.pdf).

 # Examples

The best way to use this library is to use the Servo struct.
For example, this how you can set a servomotor (id : 0x40) into continuous rotation.

```rust
extern crate drs_0x01;
use drs_0x01::prelude::Servo;

fn main() {
    let servo = Servo::new(0x40);
    let message = servo.set_speed(512);
    // ... send the message
}
```

There is also some more advanced type for user experienced with the herkulex datasheet.

This is how to do the same task (set a servomotor into continuous rotation), but using those types. Moreover we 
changed the color from blue to red to show how those types give more control.

```rust
extern crate drs_0x01;
use drs_0x01::prelude::*;
use drs_0x01::advanced::MessageBuilder;

fn main() {
    let message = MessageBuilder::new().id(0x40).s_jog(/* Playtime : datasheet value : */ 60, 
                                                       JogMode::continuous{speed : 512}, 
                                                       JogColor::red, 
                                                       0x40)
                                                 .build();
    // ... send the message.
}


```

 To reboot all the servomotors you can use this message :

 ```rust
 extern crate drs_0x01;
 use drs_0x01::advanced::MessageBuilder;
 
 fn main () {
    let message = MessageBuilder::new().id(0xFE).reboot().build();  // 0xFE is the broadcast ID
 }
 ```

 Here is how to enable torque for the servomotor labelled 35 :

 ```rust
 extern crate drs_0x01;
 use drs_0x01::advanced::MessageBuilder;
 use drs_0x01::addr::WritableRamAddr::TorqueControl;
 fn main() {
    let message = MessageBuilder::new_with_id(35).write_ram(TorqueControl(1)).build();
 }
 ```
