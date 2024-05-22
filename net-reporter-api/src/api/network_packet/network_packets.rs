use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;
use ion_rs::ReaderBuilder;
use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

use super::network_packet::NetworkPacketDTO;

const DATA_TYPE: &str = "network-packets";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct NetworkPacketsDTO {
    networks_packets: Vec<NetworkPacketDTO>,
}
impl API for NetworkPacketsDTO { }

impl NetworkPacketsDTO {
    pub fn new(network_packets: &[NetworkPacketDTO]) -> Self {
        Self { networks_packets: network_packets.to_vec() }
    }

    pub fn get_network_packets(&self) -> &[NetworkPacketDTO] {
        &self.networks_packets
    }
}

impl Encoder for NetworkPacketsDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("network_packets");
        writer.step_in(IonType::List).unwrap();
        self.networks_packets.iter().for_each(|network_packet| writer.write_blob(network_packet.encode()).unwrap());
        writer.step_out().unwrap();
        
        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkPacketsDTO {
    #[allow(unused_variables)]
    fn decode(data: &[u8]) -> NetworkPacketsDTO {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut networks = Vec::<NetworkPacketDTO>::with_capacity(elements.len());
        for element in elements {
            networks.push(NetworkPacketDTO::decode(element.as_blob().unwrap()));
        }
        binary_user_reader.step_out().unwrap();

        NetworkPacketsDTO::new(&networks)
    }
}

impl Typed for NetworkPacketsDTO {
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

    use crate::api::network_packet::network_packet::NetworkPacketDTO;

    use super::NetworkPacketsDTO;

    fn get_network_packets() -> Vec<NetworkPacketDTO>{
        const ID: &str = "1";
        const NETWORK_ID: &str = "network_id"; 
        const FRAME_TIME: i64 = i64::MIN;
        const SRC_ADDR: &str = "0.0.0.0:0000";
        const DST_ADDR: &str = "0.0.0.0:5656";
        let protocols: &[String] = &["protocol1".to_string(), "protocol2".to_string()];
        const NETWORK_PACKET_DATA: &[u8] = "NETWORK_PACKET_DATA".as_bytes();
        let network_packet = NetworkPacketDTO::new(
            ID,
            Some(NETWORK_ID),
            FRAME_TIME,
            SRC_ADDR,
            DST_ADDR,
            protocols,
            NETWORK_PACKET_DATA
        );

        const ID2: &str = "2";
        const NETWORK_ID_2: &str = "network_id_2"; 
        const FRAME_TIME_2: i64 = i64::MAX;
        const SRC_ADDR_2: &str = "0.0.0.0:0002";
        const DST_ADDR_2: &str = "0.0.0.0:5652";
        let protocols_2: &[String] = &["protocol3".to_string(), "protocol4".to_string()];
        const NETWORK_PACKET_DATA_2: &[u8] = "NETWORK_PACKET_DATA_2".as_bytes();
        let network_packet_2 = NetworkPacketDTO::new(
            ID2,
            Some(NETWORK_ID_2),
            FRAME_TIME_2,
            SRC_ADDR_2,
            DST_ADDR_2,
            protocols_2,
            NETWORK_PACKET_DATA_2
        );
        vec![network_packet, network_packet_2]
    }

    #[test]
    fn reader_correctly_read_encoded_network_packets_request() {
        let networks = get_network_packets();
        let get_networks_response = NetworkPacketsDTO::new(&networks);
        let mut binary_user_reader = ReaderBuilder::new().build(get_networks_response.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("network_packets", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        for network in networks {
            assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
            assert_eq!(network, NetworkPacketDTO::decode(binary_user_reader.read_blob().unwrap().as_slice()));
        }
        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_network_packets_request() {
        let networks = get_network_packets();
        let get_networks_response = NetworkPacketsDTO::new(&networks);
        assert_eq!(get_networks_response, NetworkPacketsDTO::decode(&get_networks_response.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        let networks = get_network_packets();
        let get_networks_response = NetworkPacketsDTO::new(&networks);
        assert_eq!(get_networks_response.get_type(), NetworkPacketsDTO::get_data_type());
        assert_eq!(get_networks_response.get_type(), super::DATA_TYPE);
    }
}
