use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;
use ion_rs::ReaderBuilder;
use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "networks-request";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct NetworksRequestDTO {
    // empty vector means we want to get all the networks
    networks_ids_to_get: Vec<String>,
}
impl API for NetworksRequestDTO { }

impl NetworksRequestDTO {
    pub fn new(networks_ids_to_get: &[String]) -> Self {
        Self { networks_ids_to_get: networks_ids_to_get.to_vec() }
    }

    pub fn get_networks_ids_to_get(&self) -> &[String] {
        &self.networks_ids_to_get
    }
}

impl Encoder for NetworksRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("networks_ids_to_get");
        writer.step_in(IonType::List).unwrap();
        self.networks_ids_to_get.iter().for_each(|network_id| writer.write_string(network_id).unwrap());
        writer.step_out().unwrap();
        
        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworksRequestDTO {
    #[allow(unused_variables)]
    fn decode(data: &[u8]) -> NetworksRequestDTO {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut network_ids_to_get = Vec::<String>::with_capacity(elements.len());
        for element in elements {
            network_ids_to_get.push(element.as_string().unwrap().to_string());
        }
        binary_user_reader.step_out().unwrap();

        NetworksRequestDTO::new(&network_ids_to_get)
    }
}

impl Typed for NetworksRequestDTO {
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

    use crate::api::network::networks_request::NetworksRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_network_request() {
        const NETWORK_IDS_TO_GET: [&str; 3] = ["1", "2", "3"];
        let get_networks_request = NetworksRequestDTO::new(NETWORK_IDS_TO_GET.map(|id| id.to_string()).as_slice());
        let mut binary_user_reader = ReaderBuilder::new().build(get_networks_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("networks_ids_to_get", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        for network_id in NETWORK_IDS_TO_GET {
            assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
            assert_eq!(network_id, binary_user_reader.read_string().unwrap().text());
        }
        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_network_request() {
        const NETWORK_IDS_TO_GET: [&str; 3] = ["1", "2", "3"];
        let get_networks_request = NetworksRequestDTO::new(NETWORK_IDS_TO_GET.map(|id| id.to_string()).as_slice());
        assert_eq!(get_networks_request, NetworksRequestDTO::decode(&get_networks_request.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const NETWORK_IDS_TO_GET: [&str; 3] = ["1", "2", "3"];
        let get_networks_request = NetworksRequestDTO::new(NETWORK_IDS_TO_GET.map(|id| id.to_string()).as_slice());
        assert_eq!(get_networks_request.get_type(), NetworksRequestDTO::get_data_type());
        assert_eq!(get_networks_request.get_type(), super::DATA_TYPE);
    }
}
