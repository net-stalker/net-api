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

use crate::api::network_packet::network_packet::NetworkPacketDTO;

const DATA_TYPE: &str = "network-packet";

#[derive(Debug, PartialEq, Eq)]
pub struct NetworkDTO {
    id: String,
    name: String,
    color: String,
    packets: Vec<NetworkPacketDTO>,
}
impl API for NetworkDTO { }

impl NetworkDTO {
    pub fn new(
        id: &str,
        name: &str,
        color: &str,
        packets: &[NetworkPacketDTO],
    ) -> Self {
        NetworkDTO { 
            id: id.into(),
            name: name.into(),
            color: color.into(),
            packets: packets.to_vec(),
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_color(&self) -> &str {
        &self.color
    }

    pub fn get_packets(&self) -> &[NetworkPacketDTO] {
        &self.packets
    }
}

impl Encoder for NetworkDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("id");
        writer.write_string(&self.id).unwrap();
        
        writer.set_field_name("name");
        writer.write_string(&self.name).unwrap();

        writer.set_field_name("color");
        writer.write_string(&self.color).unwrap();

        writer.set_field_name("packets");
        writer.step_in(IonType::List).unwrap();
        self.packets.iter().for_each(|packet| writer.write_blob(&packet.encode()).unwrap());
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let id = binary_user_reader.read_string().unwrap();

        binary_user_reader.next().unwrap();
        let name = binary_user_reader.read_string().unwrap();
        
        binary_user_reader.next().unwrap();
        let color = binary_user_reader.read_string().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut packets = Vec::<NetworkPacketDTO>::with_capacity(elements.len());
        for element in elements {
            packets.push(NetworkPacketDTO::decode(element.as_blob().unwrap()));
        }
        binary_user_reader.step_out().unwrap();

        NetworkDTO::new(
            id.text(),
            name.text(), 
            color.text(), 
            &packets, 
        )
    }
}

impl Typed for NetworkDTO {
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
    use crate::api::network::network::NetworkDTO;

    fn get_network_packets() -> Vec<NetworkPacketDTO> {
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
        vec![network_packet, network_packet2]
    }

    #[test]
    fn reader_correctly_read_encoded_network_packet() {
        const NETWORK_ID: &str = "network_id";
        const NAME: &str = "my network name";
        const COLOR: &str = "color";
        let packets = get_network_packets();
        let network = NetworkDTO::new(
            NETWORK_ID,
            NAME,
            COLOR,
            &packets
        );
        let mut binary_user_reader = ReaderBuilder::new().build(network.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("id", binary_user_reader.field_name().unwrap());
        assert_eq!(NETWORK_ID, binary_user_reader.read_string().unwrap().text());
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("name", binary_user_reader.field_name().unwrap());
        assert_eq!(NAME, binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("color", binary_user_reader.field_name().unwrap());
        assert_eq!(COLOR, binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("packets", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        for packet in packets {
            assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
            assert_eq!(packet, NetworkPacketDTO::decode(binary_user_reader.read_blob().unwrap().as_slice()));
        }
        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_network_packet() {
        const NETWORK_ID: &str = "network_id";
        const NAME: &str = "my network name";
        const COLOR: &str = "color";
        let packets = get_network_packets();
        let network = NetworkDTO::new(
            NETWORK_ID,
            NAME,
            COLOR,
            &packets
        );
        assert_eq!(network, NetworkDTO::decode(&network.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const NETWORK_ID: &str = "network_id";
        const NAME: &str = "my network name";
        const COLOR: &str = "color";
        let packets = get_network_packets();
        let network = NetworkDTO::new(
            NETWORK_ID,
            NAME,
            COLOR,
            &packets
        );
        assert_eq!(network.get_type(), NetworkDTO::get_data_type());
        assert_eq!(network.get_type(), super::DATA_TYPE);
    }
}