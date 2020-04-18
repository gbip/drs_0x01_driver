[![Latest version](https://img.shields.io/crates/v/drs-0x01.svg)](https://crates.io/crates/drs-0x01)
[![Documentation](https://docs.rs/drs-0x01/badge.svg)](https://docs.rs/drs-0x01)
# [Documentation](https://docs.rs/drs-0x01/0.1.6/drs_0x01/)
 
 This crate provides basic functionalities to communicate with Herkulex DRS (both 0101 and 0201)
 servomotors.
 It is heavily based on the documentation published by Dongbu Robot which is available
 [`here`](http://www.sgbotic.com/products/datasheets/robotics/herkulexeng.pdf).

## Crate maturity

This library has been used successfully to drive servomotors on robot designed to compete at Eurobot. So the emitting side of the library is tested and should be bug free. However, the receiving side of the library isn't as much tested. To be tagged as `1.0` more efforts should be put into the test suite.

## Examples

[Herkulex Manager](https://git.florencepaul.com/gbip/herkulex_manager) is a binary CLI to send commands to the servomotors. It can be used as an example on how to integrate the library inside a bigger application.

### Sending data
The best way to use this library to send data to the servomotor is to use the Servo struct.
For example, this how you can set a servomotor (id : 0x40) into continuous rotation.

```rust
extern crate drs_0x01;
use drs_0x01::Servo;

fn main() {
    let servo = Servo::new(0x40);
    let message = servo.set_speed(512, Rotation::Clockwise);
    // ... send the message
}
```

There is also some more advanced type for user experienced with the herkulex datasheet.

This is how to do the same task (set a servomotor into continuous rotation), but using those types. Moreover we 
changed the color from blue to red to show how those types give more control.

```rust
extern crate drs_0x01;
use drs_0x01::*;
use drs_0x01::builder::MessageBuilder;

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
 use drs_0x01::builder::MessageBuilder;
 
 fn main () {
    let message = MessageBuilder::new().id(0xFE).reboot().build();  // 0xFE is the broadcast ID
 }
 ```

 Here is how to enable torque for the servomotor labelled 35 :

 ```rust
 extern crate drs_0x01;
 use drs_0x01::builder::MessageBuilder;
 use drs_0x01::WritableRamAddr::TorqueControl;
 fn main() {
    let message = MessageBuilder::new_with_id(35).write_ram(TorqueControl(1)).build();
 }
 ```
 
 ### Receiving Data
 
 You can easily parse incoming bytes and transform them into [ACKPacket](https://docs.rs/drs-0x01/latest/drs_0x01/reader/struct.ACKPacket.html) 
 by using an [ACKReader](https://docs.rs/drs-0x01/latest/drs_0x01/reader/struct.ACKReader.html).

From [`examples/read_status.rs`](examples/read_status.rs) :

```rust
extern crate drs_0x01;
use drs_0x01::builder::MessageBuilder;
use drs_0x01::reader::ACKReader;
fn main() {
    let mut reader = ACKReader::new();

    let _message = MessageBuilder::new_with_id(35).stat().build();
    // Send the message ...
    let received_message = [0u8];
    reader.parse(&received_message);
    if let Some(_packet) = reader.pop_ack_packet() {
        // Handle the packet
    }
}
```
