use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "protocol";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProtocolDTO {
    name: String,
    total_bytes: i64,
}
impl API for ProtocolDTO { }

impl ProtocolDTO {
    pub fn new(name: &str, total_bytes: i64) -> Self {
        Self {
            name: name.to_string(),
            total_bytes
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn get_total_bytes(&self) -> i64 {
        self.total_bytes
    }
}

impl Encoder for ProtocolDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("name");
        writer.write_string(&self.name).unwrap();

        writer.set_field_name("total_bytes");
        writer.write_i64(self.total_bytes).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for ProtocolDTO {
    fn decode(data: &[u8]) -> Self where Self: Sized  {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let name = binding.text();

        binary_user_reader.next().unwrap();
        let total_bytes = binary_user_reader.read_i64().unwrap();

        ProtocolDTO::new(
            name,
            total_bytes,
        )
    }
}

impl Typed for ProtocolDTO {
    fn get_data_type() -> &'static str where Self: Sized {
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

    use crate::api::network_bandwidth_per_protocol::protocol::ProtocolDTO;


    #[test]
    fn reader_correctly_read_encoded_protocol() {
        const NAME: &str = "PROTOCOL_NAME";
        const TOTAL_BYTES: i64 = i64::MAX;

        let protocol = ProtocolDTO::new(NAME, TOTAL_BYTES);
        let mut binary_user_reader = ReaderBuilder::new().build(protocol.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("name", binary_user_reader.field_name().unwrap());
        assert_eq!(NAME, binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("total_bytes", binary_user_reader.field_name().unwrap());
        assert_eq!(TOTAL_BYTES, binary_user_reader.read_i64().unwrap());
    }

    #[test]
    fn endec_protocol() {
        const NAME: &str = "PROTOCOL_NAME";
        const TOTAL_BYTES: i64 = i64::MAX;

        let protocol = ProtocolDTO::new(NAME, TOTAL_BYTES);
        assert_eq!(protocol, ProtocolDTO::decode(&protocol.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const NAME: &str = "PROTOCOL_NAME";
        const TOTAL_BYTES: i64 = i64::MAX;

        let protocol = ProtocolDTO::new(NAME, TOTAL_BYTES);
        assert_eq!(protocol.get_type(), ProtocolDTO::get_data_type());
        assert_eq!(protocol.get_type(), super::DATA_TYPE);
    }
}