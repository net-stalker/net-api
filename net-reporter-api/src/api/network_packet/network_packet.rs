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

const DATA_TYPE: &str = "network-packet";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NetworkPacketDTO {
    id: String,
    network_id: Option<String>,
    frame_time: i64,
    src: String,
    dst: String,
    protocols: Vec<String>,
    json_data: Vec<u8>,
}
impl API for NetworkPacketDTO { }

impl NetworkPacketDTO {
    pub fn new(
        id: &str,
        network_id: Option<&str>,
        frame_time: i64,
        src: &str,
        dst: &str,
        protocols: &[String],
        json_data: &[u8],
    ) -> Self {
        NetworkPacketDTO { 
            id: id.to_string(),
            network_id: network_id.map(str::to_string),
            frame_time, 
            src: src.into(), 
            dst: dst.into(), 
            protocols: protocols.to_vec(),
            json_data: json_data.into(),
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_network_id(&self) -> Option<&str> {
        self.network_id.as_deref()
    }

    pub fn get_frame_time(&self) -> i64 {
        self.frame_time
    }

    pub fn get_src(&self) -> &str {
        &self.src
    }

    pub fn get_dst(&self) -> &str {
        &self.dst
    }

    pub fn get_protocols(&self) -> &[String] {
        &self.protocols
    }

    pub fn get_json(&self) -> &[u8] {
        &self.json_data
    }
}

impl Encoder for NetworkPacketDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("id");
        writer.write_string(&self.id).unwrap();

        writer.set_field_name("network_id");
        match self.network_id.as_ref() {
            Some(network_id) => writer.write_string(network_id).unwrap(),
            None => writer.write_null(IonType::String).unwrap(),
        };
        
        writer.set_field_name("frame_time");
        writer.write_i64(self.frame_time).unwrap();

        writer.set_field_name("src");
        writer.write_string(&self.src).unwrap();

        writer.set_field_name("dst");
        writer.write_string(&self.dst).unwrap();

        writer.set_field_name("protocols");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        for protocol in &self.protocols {
            writer.write_string(protocol).unwrap();
        }
        writer.step_out().unwrap();
        
        writer.set_field_name("json_data");
        writer.write_blob(&self.json_data).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkPacketDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();let id = binary_user_reader.read_string().unwrap();

        binary_user_reader.next().unwrap();
        let network_id = match binary_user_reader.current() {
            ion_rs::StreamItem::Value(_) => Some(binary_user_reader.read_string().unwrap().text().to_string()),
            _ => None,
        };

        binary_user_reader.next().unwrap();
        let frame_time = binary_user_reader.read_i64().unwrap();
        
        binary_user_reader.next().unwrap();
        let src = binary_user_reader.read_string().unwrap();

        binary_user_reader.next().unwrap();
        let dst = binary_user_reader.read_string().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut protocols = Vec::<String>::with_capacity(elements.len());
        for element in elements {
            protocols.push(element.as_text().unwrap().to_string());
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_blob().unwrap();
        let json_data = binding.as_slice();

        NetworkPacketDTO::new(
            id.text(),
            network_id.as_deref(),
            frame_time, 
            src.text(), 
            dst.text(),
            &protocols, 
            json_data,
        )
    }
}

impl Typed for NetworkPacketDTO {
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

    #[test]
    fn reader_correctly_read_encoded_network_packet() {
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
        let mut binary_user_reader = ReaderBuilder::new().build(network_packet.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("id", binary_user_reader.field_name().unwrap());
        assert_eq!(ID, binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("network_id", binary_user_reader.field_name().unwrap());
        assert_eq!(NETWORK_ID, binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("frame_time", binary_user_reader.field_name().unwrap());
        assert_eq!(FRAME_TIME, binary_user_reader.read_i64().unwrap());
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("src", binary_user_reader.field_name().unwrap());
        assert_eq!(SRC_ADDR, binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("dst", binary_user_reader.field_name().unwrap());
        assert_eq!(DST_ADDR, binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("protocols", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        for protocol in protocols {
            assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
            assert_eq!(protocol, binary_user_reader.read_string().unwrap().text());
        }
        binary_user_reader.step_out().unwrap();

        assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
        assert_eq!("json_data", binary_user_reader.field_name().unwrap());
        assert_eq!(NETWORK_PACKET_DATA, binary_user_reader.read_blob().unwrap().as_slice());
    }

    #[test]
    fn endec_network_packet_some_network() {
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
        assert_eq!(network_packet, NetworkPacketDTO::decode(&network_packet.encode()));
    }

    #[test]
    fn endec_network_packet_none_network() {
        const ID: &str = "1";
        const FRAME_TIME: i64 = i64::MIN;
        const SRC_ADDR: &str = "0.0.0.0:0000";
        const DST_ADDR: &str = "0.0.0.0:5656";
        let protocols: &[String] = &["protocol1".to_string(), "protocol2".to_string()];
        const NETWORK_PACKET_DATA: &[u8] = "NETWORK_PACKET_DATA".as_bytes();
        let network_packet = NetworkPacketDTO::new(
            ID,
            None,
            FRAME_TIME,
            SRC_ADDR,
            DST_ADDR,
            protocols,
            NETWORK_PACKET_DATA
        );
        assert_eq!(network_packet, NetworkPacketDTO::decode(&network_packet.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const ID: &str = "1";
        const FRAME_TIME: i64 = i64::MIN;
        const SRC_ADDR: &str = "0.0.0.0:0000";
        const DST_ADDR: &str = "0.0.0.0:5656";
        let protocols: &[String] = &["protocol1".to_string(), "protocol2".to_string()];
        const NETWORK_PACKET_DATA: &[u8] = "NETWORK_PACKET_DATA".as_bytes();
        let network_packet = NetworkPacketDTO::new(
            ID,
            None,
            FRAME_TIME,
            SRC_ADDR,
            DST_ADDR,
            protocols,
            NETWORK_PACKET_DATA
        );
        assert_eq!(network_packet.get_type(), NetworkPacketDTO::get_data_type());
        assert_eq!(network_packet.get_type(), super::DATA_TYPE);
    }
}