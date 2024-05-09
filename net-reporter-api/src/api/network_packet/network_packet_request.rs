use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "network-packet-request";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NetworkPacketRequestDTO {
    id: String,
}
impl API for NetworkPacketRequestDTO { }

impl NetworkPacketRequestDTO {
    pub fn new(id: &str) -> Self {
        NetworkPacketRequestDTO { 
            id: id.into(),
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

impl Encoder for NetworkPacketRequestDTO {
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

impl Decoder for NetworkPacketRequestDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let id = binary_user_reader.read_string().unwrap();

        NetworkPacketRequestDTO::new(id.text())
    }
}

impl Typed for NetworkPacketRequestDTO {
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
    use net_core_api::core::typed_api::Typed;

    use crate::api::network_packet::network_packet_request::NetworkPacketRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_network_packet_request() {
        const ID: &str = "1";
        let network_packet_request = NetworkPacketRequestDTO::new(ID);
        let mut binary_user_reader = ReaderBuilder::new().build(network_packet_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("id", binary_user_reader.field_name().unwrap());
        assert_eq!(ID, binary_user_reader.read_string().unwrap().text());
    }

    #[test]
    fn endec_network_packet_request() {
        const ID: &str = "1";
        let network_packet_request = NetworkPacketRequestDTO::new(ID);
        assert_eq!(network_packet_request, NetworkPacketRequestDTO::decode(&network_packet_request.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const ID: &str = "1";
        let network_packet_request = NetworkPacketRequestDTO::new(ID);
        assert_eq!(network_packet_request.get_type(), NetworkPacketRequestDTO::get_data_type());
        assert_eq!(network_packet_request.get_type(), super::DATA_TYPE);
    }
}