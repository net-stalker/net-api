use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;
use ion_rs::ReaderBuilder;
use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

use super::network::NetworkDTO;

const DATA_TYPE: &str = "networks-response";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct NetworksResponseDTO {
    networks: Vec<NetworkDTO>,
}
impl API for NetworksResponseDTO { }

impl NetworksResponseDTO {
    pub fn new(networks: &[NetworkDTO]) -> Self {
        Self { networks: networks.to_vec() }
    }

    pub fn get_networks(&self) -> &[NetworkDTO] {
        &self.networks
    }
}

impl Encoder for NetworksResponseDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("networks");
        writer.step_in(IonType::List).unwrap();
        self.networks.iter().for_each(|network| writer.write_blob(network.encode()).unwrap());
        writer.step_out().unwrap();
        
        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworksResponseDTO {
    #[allow(unused_variables)]
    fn decode(data: &[u8]) -> NetworksResponseDTO {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut networks = Vec::<NetworkDTO>::with_capacity(elements.len());
        for element in elements {
            networks.push(NetworkDTO::decode(element.as_blob().unwrap()));
        }
        binary_user_reader.step_out().unwrap();

        NetworksResponseDTO::new(&networks)
    }
}

impl Typed for NetworksResponseDTO {
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

    use crate::api::network::network::NetworkDTO;
    use crate::api::network_packet::network_packet::NetworkPacketDTO;

    use super::NetworksResponseDTO;

    fn get_networks() -> Vec<NetworkDTO>{
        const ID: &str = "1";
        const FRAME_TIME: i64 = i64::MIN;
        const SRC_ADDR: &str = "0.0.0.0:0000";
        const DST_ADDR: &str = "0.0.0.0:5656";
        let protocols: &[String] = &["protocol1".to_string(), "protocol2".to_string()];
        const NETWORK_PACKET_DATA: &[u8] = "NETWORK_PACKET_DATA".as_bytes();
        let network_packet = NetworkPacketDTO::new(
            ID,
            FRAME_TIME,
            SRC_ADDR,
            DST_ADDR,
            protocols,
            NETWORK_PACKET_DATA
        );
        const ID2: &str = "2";
        const FRAME_TIME2: i64 = i64::MIN;
        const SRC_ADDR2: &str = "0.1.0.0:0000";
        const DST_ADDR2: &str = "0.2.0.0:5656";
        let protocols2: &[String] = &["protocol3".to_string(), "protocol4".to_string()];
        const NETWORK_PACKET_DATA2: &[u8] = "NETWORK_PACKET_DATA_2".as_bytes();
        let network_packet2 = NetworkPacketDTO::new(
            ID2,
            FRAME_TIME2,
            SRC_ADDR2,
            DST_ADDR2,
            protocols2,
            NETWORK_PACKET_DATA2
        );
        let first_pair = vec![network_packet.clone(), network_packet2.clone()];
        let second_pair = vec![network_packet2, network_packet];
        const NETWORK_ID1: &str = "network_id1";
        const NAME1: &str = "network name 1";
        const COLOR1: &str = "network color 1";

        const NETWORK_ID2: &str = "network_id2";
        const NAME2: &str = "network name 2";
        const COLOR2: &str = "network color 2";
        vec![NetworkDTO::new(NETWORK_ID1, NAME1, COLOR1, &first_pair), NetworkDTO::new(NETWORK_ID2, NAME2, COLOR2, &second_pair)]
    }

    #[test]
    fn reader_correctly_read_encoded_network_request() {
        let networks = get_networks();
        let get_networks_response = NetworksResponseDTO::new(&networks);
        let mut binary_user_reader = ReaderBuilder::new().build(get_networks_response.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("networks", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        for network in networks {
            assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
            assert_eq!(network, NetworkDTO::decode(binary_user_reader.read_blob().unwrap().as_slice()));
        }
        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_network_request() {
        let networks = get_networks();
        let get_networks_response = NetworksResponseDTO::new(&networks);
        assert_eq!(get_networks_response, NetworksResponseDTO::decode(&get_networks_response.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        let networks = get_networks();
        let get_networks_response = NetworksResponseDTO::new(&networks);
        assert_eq!(get_networks_response.get_type(), NetworksResponseDTO::get_data_type());
        assert_eq!(get_networks_response.get_type(), super::DATA_TYPE);
    }
}
