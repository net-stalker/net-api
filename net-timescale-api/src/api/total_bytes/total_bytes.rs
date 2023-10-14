use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


const DATA_TYPE: &str = "total-bytes";
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TotalBytesDTO {
    endpoint: String,
    total_bytes: u64
    // TODO: think about proving agent_id here
}
//
// impl TotalBytesDTO {
//     pub fn new(endpoint: &str, total_bytes)
// }