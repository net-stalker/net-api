use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use ion_rs::element::reader::ElementReader;

use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

use super::protocol::ProtocolDTO;


const DATA_TYPE: &str = "network_bandwidth_per_protocol";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NetworkBandwidthPerProtocolDTO {
    protocols: Vec<ProtocolDTO>,
}
impl API for  NetworkBandwidthPerProtocolDTO { }

impl NetworkBandwidthPerProtocolDTO {
    pub fn new(protocols: &[ProtocolDTO]) -> Self {
        Self { protocols: protocols.to_vec() }
    }

    pub fn get_protocols(&self) -> &[ProtocolDTO] {
        self.protocols.as_slice()
    }
}

impl Encoder for NetworkBandwidthPerProtocolDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("protocols");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        self.protocols.iter().for_each(|protocol| {
            let data = protocol.encode();
            writer.write_blob(data.as_slice()).unwrap();
        });
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkBandwidthPerProtocolDTO {
    fn decode(data: &[u8]) -> Self where Self: Sized {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let protocol_elements = binary_user_reader.read_all_elements().unwrap();
        let mut protocols = Vec::with_capacity(protocol_elements.len());
        protocol_elements.iter().for_each(|protocol| {
            let data = protocol.as_blob().unwrap();
            let protocol = ProtocolDTO::decode(data);
            protocols.push(protocol);
        });

        Self::new(
            &protocols
        )
    }
}

impl Typed for NetworkBandwidthPerProtocolDTO {
    fn get_data_type() -> &'static str where Self: Sized {
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

    use crate::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol::NetworkBandwidthPerProtocolDTO;
    use crate::api::network_bandwidth_per_protocol::protocol::ProtocolDTO;

    #[test]
    fn test_correctly_read_encoded_bandwidth_per_protocol() {
        const FIRST_NAME: &str = "FIRST_PROTOCOL_NAME";
        const FIRST_TOTAL_BYTES: i64 = i64::MIN;

        const SECOND_NAME: &str = "SECOND_PROTOCOL_NAME";
        const SECOND_TOTAL_BYTES: i64 = i64::MAX;

        let protocols = vec![
            ProtocolDTO::new(FIRST_NAME, FIRST_TOTAL_BYTES),
            ProtocolDTO::new(SECOND_NAME, SECOND_TOTAL_BYTES),
        ];

        let network_bandwidth_per_protocol = NetworkBandwidthPerProtocolDTO::new(protocols.as_slice());

        let mut binary_user_reader = ReaderBuilder::new().build(network_bandwidth_per_protocol.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!(binary_user_reader.field_name().unwrap(), "protocols");
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        assert_eq!(elements.len(), protocols.len());
        for (element, protocol_core) in elements.iter().zip(protocols.as_slice()) {
            let encoded_protocol = ProtocolDTO::decode(element.as_blob().unwrap());
            assert_eq!(encoded_protocol, *protocol_core);
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn test_endec_bandwidth_per_protocol() {
        const FIRST_NAME: &str = "FIRST_PROTOCOL_NAME";
        const FIRST_TOTAL_BYTES: i64 = i64::MIN;

        const SECOND_NAME: &str = "SECOND_PROTOCOL_NAME";
        const SECOND_TOTAL_BYTES: i64 = i64::MAX;

        let protocols = vec![
            ProtocolDTO::new(FIRST_NAME, FIRST_TOTAL_BYTES),
            ProtocolDTO::new(SECOND_NAME, SECOND_TOTAL_BYTES),
        ];

        let network_bandwidth_per_protocol = NetworkBandwidthPerProtocolDTO::new(protocols.as_slice());

        assert_eq!(network_bandwidth_per_protocol, NetworkBandwidthPerProtocolDTO::decode(network_bandwidth_per_protocol.encode().as_slice()));
    }

    #[test]
    fn test_getting_data_types() {
        const FIRST_NAME: &str = "FIRST_PROTOCOL_NAME";
        const FIRST_TOTAL_BYTES: i64 = i64::MIN;

        const SECOND_NAME: &str = "SECOND_PROTOCOL_NAME";
        const SECOND_TOTAL_BYTES: i64 = i64::MAX;

        let protocols = vec![
            ProtocolDTO::new(FIRST_NAME, FIRST_TOTAL_BYTES),
            ProtocolDTO::new(SECOND_NAME, SECOND_TOTAL_BYTES),
        ];

        let network_bandwidth_per_protocol = NetworkBandwidthPerProtocolDTO::new(protocols.as_slice());
        assert_eq!(network_bandwidth_per_protocol.get_type(), NetworkBandwidthPerProtocolDTO::get_data_type());
        assert_eq!(network_bandwidth_per_protocol.get_type(), super::DATA_TYPE);
    }
}