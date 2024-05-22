use ion_rs;

use ion_rs::element::reader::ElementReader;
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
pub struct NetworkPacketsRequestDTO {
    packets_networks_ids: Vec<Option<String>>,
}
impl API for NetworkPacketsRequestDTO { }

impl NetworkPacketsRequestDTO {
    pub fn new(packets_networks_ids: &[Option<&str>]) -> Self {
        NetworkPacketsRequestDTO { 
            packets_networks_ids: packets_networks_ids.iter().map(|id| id.map(str::to_string)).collect(),
        }
    }
    pub fn get_id(&self) -> &[Option<String>] {
        self.packets_networks_ids.as_slice()
    }
}

impl Encoder for NetworkPacketsRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("packets_networks_ids");
        writer.step_in(IonType::List).unwrap();
        self.packets_networks_ids.iter().for_each(|network_id|{
            match network_id.as_ref() {
                Some(id) => writer.write_string(id).unwrap(),
                None => writer.write_null(IonType::String).unwrap(),
            }
        });
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkPacketsRequestDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        let elements = binary_user_reader.read_all_elements().unwrap();
        let ids: Vec<Option<&str>> = elements.iter().map(|element| element.as_text()).collect();

        NetworkPacketsRequestDTO::new(ids.as_slice())
    }
}

impl Typed for NetworkPacketsRequestDTO {
    fn get_data_type() -> &'static str {
        DATA_TYPE
    }

    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}


#[cfg(test)]
mod tests {
    use ion_rs::element::reader::ElementReader;
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use net_core_api::core::encoder_api::Encoder;
    use net_core_api::core::decoder_api::Decoder;
    use net_core_api::core::typed_api::Typed;

    use crate::api::network_packet::network_packets_request::NetworkPacketsRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_network_packets_request() {
        let test_networks_ids = vec![Some("1"), Some("2"), None, None, Some("3")];
        let network_packets_request = NetworkPacketsRequestDTO::new(test_networks_ids.as_slice());
        let mut binary_user_reader = ReaderBuilder::new().build(network_packets_request.encode()).unwrap();
        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("packets_networks_ids", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        for (element, id) in elements.iter().zip(test_networks_ids.iter()) {
            assert_eq!(element.as_text(), *id);
        }
        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_network_packet_request() {
        let test_networks_ids = vec![Some("1"), Some("2"), None, None, Some("3")];
        let network_packets_request = NetworkPacketsRequestDTO::new(test_networks_ids.as_slice()); 
        assert_eq!(network_packets_request, NetworkPacketsRequestDTO::decode(&network_packets_request.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        let test_networks_ids = vec![Some("1"), Some("2"), None, None, Some("3")];
        let network_packets_request = NetworkPacketsRequestDTO::new(test_networks_ids.as_slice()); 
        assert_eq!(network_packets_request.get_type(), NetworkPacketsRequestDTO::get_data_type());
        assert_eq!(network_packets_request.get_type(), super::DATA_TYPE);
    }
}