use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "delete-buffered-packet-request";

#[derive(Debug, PartialEq, Eq)]
pub struct DeleteBufferedPacketRequestDTO {
    id: String,
}
impl API for DeleteBufferedPacketRequestDTO { }

impl DeleteBufferedPacketRequestDTO {
    pub fn new(
        id: &str
    ) -> Self {
        DeleteBufferedPacketRequestDTO {
            id: id.to_string(),
        }
    }
    
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

impl Encoder for DeleteBufferedPacketRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("id");
        writer.write_string(&self.id).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for DeleteBufferedPacketRequestDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let id = binary_user_reader.read_string().unwrap();
        
        DeleteBufferedPacketRequestDTO::new(id.text())
    }
}

impl Typed for DeleteBufferedPacketRequestDTO {
    fn get_data_type() -> &'static str {
        DATA_TYPE
    }

    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}

#[cfg(test)]
mod tests {
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use net_core_api::core::encoder_api::Encoder;
    use net_core_api::core::decoder_api::Decoder;

    use crate::api::buffered_packet::DeleteBufferedPacketRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_delete_buffered_packet_request() {
        const ID: &str = "1";
        let delete_packet_request = DeleteBufferedPacketRequestDTO::new(ID);
        let mut binary_user_reader = ReaderBuilder::new().build(delete_packet_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("id", binary_user_reader.field_name().unwrap());
        assert_eq!(ID, binary_user_reader.read_str().unwrap());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_delete_buffered_packet_request() {
        const ID: &str = "1";
        let delete_packet_request = DeleteBufferedPacketRequestDTO::new(ID);
        
        assert_eq!(delete_packet_request, DeleteBufferedPacketRequestDTO::decode(&delete_packet_request.encode()));
    }
}
