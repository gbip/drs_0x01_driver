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
        // Handle the packer
    }
}
