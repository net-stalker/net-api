use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;


const DATA_TYPE: &str = "endpoint";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EndpointDTO {
    id: String,
    total_bytes_received: i64,
    total_bytes_sent: i64,
    // TODO: think about proving agent_id here
}
impl API for EndpointDTO { }

impl EndpointDTO {
    pub fn new(id: &str, total_bytes_received: i64, total_bytes_sent: i64) -> Self {
        Self {
            id: id.to_string(),
            total_bytes_received,
            total_bytes_sent,
        }
    }
    pub fn m_new(id: String, total_bytes_received: i64, total_bytes_sent: i64) -> Self {
        Self {
            id,
            total_bytes_received,
            total_bytes_sent,
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
    pub fn get_total_bytes_received(&self) -> i64 {
        self.total_bytes_received
    }
    pub fn get_total_bytes_sent(&self) -> i64 {
        self.total_bytes_sent
    }
}

impl Encoder for EndpointDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("id");
        writer.write_string(self.id.as_str()).unwrap();

        writer.set_field_name("total_bytes_received");
        writer.write_i64(self.total_bytes_received).unwrap();

        writer.set_field_name("total_bytes_sent");
        writer.write_i64(self.total_bytes_sent).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for EndpointDTO {
    fn decode(data: &[u8]) -> Self where Self: Sized  {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let id = binary_user_reader.read_str().unwrap().to_string();

        binary_user_reader.next().unwrap();
        let total_bytes_received = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        let total_bytes_sent = binary_user_reader.read_i64().unwrap();

        EndpointDTO::m_new(
            id,
            total_bytes_received,
            total_bytes_sent,
        )
    }
}

impl Typed for EndpointDTO {
    fn get_data_type() -> &'static str where Self: Sized {
        DATA_TYPE
    }

    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}


#[cfg(test)]
mod tests {
    use ion_rs::{IonReader, IonType, ReaderBuilder, StreamItem};

    use net_core_api::core::encoder_api::Encoder;
    use net_core_api::core::decoder_api::Decoder;
    use net_core_api::core::typed_api::Typed;

    use crate::api::network_bandwidth_per_endpoint::endpoint::EndpointDTO;

    #[test]
    fn test_correctly_read_encoded_endpoint() {
        const ID: &str = "id";
        const TOTAL_BYTES_RECEIVED: i64 = i64::MIN;
        const TOTAL_BYTES_SENT: i64 = i64::MAX;

        let endpoint = EndpointDTO::new(
            ID,
            TOTAL_BYTES_RECEIVED,
            TOTAL_BYTES_SENT
        );

        let mut binary_user_reader = ReaderBuilder::new().build(endpoint.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("id", binary_user_reader.field_name().unwrap());
        assert_eq!(ID, binary_user_reader.read_str().unwrap());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("total_bytes_received", binary_user_reader.field_name().unwrap());
        assert_eq!(TOTAL_BYTES_RECEIVED,  binary_user_reader.read_i64().unwrap());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("total_bytes_sent", binary_user_reader.field_name().unwrap());
        assert_eq!(TOTAL_BYTES_SENT,  binary_user_reader.read_i64().unwrap());
    }

    #[test]
    fn test_endec_endpoint() {
        const ID: &str = "id";
        const TOTAL_BYTES_RECEIVED: i64 = i64::MIN;
        const TOTAL_BYTES_SENT: i64 = i64::MAX;

        let endpoint = EndpointDTO::new(
            ID,
            TOTAL_BYTES_RECEIVED,
            TOTAL_BYTES_SENT
        );

       assert_eq!(endpoint, EndpointDTO::decode(endpoint.encode().as_slice()));
    }
    #[test]
    fn test_getting_data_types() {
        const ID: &str = "id";
        const TOTAL_BYTES_RECEIVED: i64 = i64::MIN;
        const TOTAL_BYTES_SENT: i64 = i64::MAX;

        let endpoint = EndpointDTO::new(
            ID,
            TOTAL_BYTES_RECEIVED,
            TOTAL_BYTES_SENT
        );
        assert_eq!(endpoint.get_type(), EndpointDTO::get_data_type());
        assert_eq!(endpoint.get_type(), super::DATA_TYPE);
    }
}