use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;
use ion_rs::ReaderBuilder;

use net_proto_api::api::API;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::typed_api::Typed;


const DATA_TYPE: &str = "network_graph_request";

#[derive(Debug, PartialEq, Eq)]
pub struct NetworkGraphRequestDTO {
    start_date_time: i64,
    end_date_time: i64,
    subscribe: bool,
}
impl API for NetworkGraphRequestDTO { }

impl NetworkGraphRequestDTO {
    pub fn new(start_date_time: i64, end_date_time: i64, subscribe: bool) -> Self {
        NetworkGraphRequestDTO {
            start_date_time,
            end_date_time,
            subscribe,
        }
    }

    pub fn get_start_date_time(&self) -> i64 {
        self.start_date_time
    }

    pub fn get_end_date_time(&self) -> i64 {
        self.end_date_time
    }

    pub fn is_subscribe(&self) -> bool {
        self.subscribe
    }
}

impl Encoder for NetworkGraphRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("start_date_time");
        writer.write_i64(self.start_date_time).unwrap();

        writer.set_field_name("end_date_time");
        writer.write_i64(self.end_date_time).unwrap();

        writer.set_field_name("subscribe");
        writer.write_bool(self.subscribe).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkGraphRequestDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let start_date_time = binary_user_reader.read_i64().unwrap();
        
        binary_user_reader.next().unwrap();
        let end_date_time = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        let subscribe = binary_user_reader.read_bool().unwrap();

        NetworkGraphRequestDTO::new(
            start_date_time,
            end_date_time,
            subscribe
        )
    }
}

impl Typed for NetworkGraphRequestDTO {
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

    use net_proto_api::decoder_api::Decoder;
    use net_proto_api::encoder_api::Encoder;
    use net_proto_api::typed_api::Typed;

    use crate::api::network_graph::network_graph_request::NetworkGraphRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_ng_request() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;
        const SUBSCRIBE: bool = true;

        let network_graph_request = NetworkGraphRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            SUBSCRIBE,
        );
        
        let mut binary_user_reader = ReaderBuilder::new().build(network_graph_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("start_date_time", binary_user_reader.field_name().unwrap());
        assert_eq!(START_DATE_TIME, binary_user_reader.read_i64().unwrap());
        
        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("end_date_time", binary_user_reader.field_name().unwrap());
        assert_eq!(END_DATE_TIME,  binary_user_reader.read_i64().unwrap());

        assert_eq!(StreamItem::Value(IonType::Bool), binary_user_reader.next().unwrap());
        assert_eq!("subscribe", binary_user_reader.field_name().unwrap());
        assert_eq!(SUBSCRIBE,  binary_user_reader.read_bool().unwrap());
    }

    #[test]
    fn endec_ng_request() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;
        const SUBSCRIBE: bool = true;

        let network_graph_request = NetworkGraphRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            SUBSCRIBE,
        );
        assert_eq!(network_graph_request, NetworkGraphRequestDTO::decode(&network_graph_request.encode()));
    }
    #[test]
    fn test_getting_data_types() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;
        const SUBSCRIBE: bool = true;

        let network_graph_request = NetworkGraphRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            SUBSCRIBE,
        );
        assert_eq!(network_graph_request.get_type(), NetworkGraphRequestDTO::get_data_type());
        assert_eq!(network_graph_request.get_type(), super::DATA_TYPE);
    }
}