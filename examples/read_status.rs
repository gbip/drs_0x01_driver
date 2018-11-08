extern crate drs_0x01;
use drs_0x01::addr::WritableRamAddr::TorqueControl;
use drs_0x01::advanced::MessageBuilder;
use drs_0x01::reader::ACKReader;
fn main() {
    let reader = ACKReader::new();

    let message = MessageBuilder::new_with_id(35).stat().build();
    // Send the message ...
    let received_message = [0u8];
}
