use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;

use super::network_bandwidth_per_protocol_filters::NetworkBandwidthPerProtocolFiltersDTO;


const DATA_TYPE: &str = "network_bandwidth_per_protocol_request";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NetworkBandwidthPerProtocolRequestDTO {
    start_date_time: i64,
    end_date_time: i64,
    filters: NetworkBandwidthPerProtocolFiltersDTO,
}
impl API for NetworkBandwidthPerProtocolRequestDTO { }

impl NetworkBandwidthPerProtocolRequestDTO {
    pub fn new(start_date_time: i64, end_date_time: i64, filters: NetworkBandwidthPerProtocolFiltersDTO) -> Self {
        NetworkBandwidthPerProtocolRequestDTO {
            start_date_time,
            end_date_time,
            filters,
        }
    }

    pub fn get_start_date_time(&self) -> i64 {
        self.start_date_time
    }

    pub fn get_end_date_time(&self) -> i64 {
        self.end_date_time
    }

    pub fn get_filters(&self) -> &NetworkBandwidthPerProtocolFiltersDTO {
        &self.filters
    }
}

impl Encoder for NetworkBandwidthPerProtocolRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("start_date_time");
        writer.write_i64(self.start_date_time).unwrap();

        writer.set_field_name("end_date_time");
        writer.write_i64(self.end_date_time).unwrap();

        writer.set_field_name("filters");
        writer.write_blob(self.filters.encode()).unwrap();
        
        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkBandwidthPerProtocolRequestDTO {
    fn decode(data: &[u8]) -> Self where Self: Sized {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let start_date_time = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        let end_date_time = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        let filters = NetworkBandwidthPerProtocolFiltersDTO::decode(binary_user_reader.read_blob().unwrap().as_slice());

        NetworkBandwidthPerProtocolRequestDTO::new(
            start_date_time,
            end_date_time,
            filters,
        )
    }
}

impl Typed for NetworkBandwidthPerProtocolRequestDTO {
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

    use net_core_api::encoder_api::Encoder;
    use net_core_api::decoder_api::Decoder;
    use net_core_api::typed_api::Typed;

    use crate::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol_filters::NetworkBandwidthPerProtocolFiltersDTO;
    use crate::api::network_bandwidth_per_protocol::network_bandwidth_per_protocol_request::NetworkBandwidthPerProtocolRequestDTO;

    fn get_test_filters() -> NetworkBandwidthPerProtocolFiltersDTO {
        let endpoints = vec!["0.0.0.0".to_string(), "1.1.1.1".to_string()];
        const INCLUDE_ENDPOINTS_MODE: bool = true;
        let bytes_lower_bound = Some(100);
        let bytes_upper_bound = Some(1000);

        NetworkBandwidthPerProtocolFiltersDTO::new(
            &endpoints,
            Some(INCLUDE_ENDPOINTS_MODE),
            bytes_lower_bound,
            bytes_upper_bound
        )
    }

    #[test]
    fn reader_correctly_read_encoded_network_bandwidth_per_protocol_request() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;

        let network_bandwidth_per_protocol_request = NetworkBandwidthPerProtocolRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            get_test_filters(),
        );

        let mut binary_user_reader = ReaderBuilder::new().build(network_bandwidth_per_protocol_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("start_date_time", binary_user_reader.field_name().unwrap());
        assert_eq!(START_DATE_TIME, binary_user_reader.read_i64().unwrap());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("end_date_time", binary_user_reader.field_name().unwrap());
        assert_eq!(END_DATE_TIME,  binary_user_reader.read_i64().unwrap());

        assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
        assert_eq!("filters", binary_user_reader.field_name().unwrap());
        assert_eq!(get_test_filters(),  NetworkBandwidthPerProtocolFiltersDTO::decode(binary_user_reader.read_blob().unwrap().as_slice()));
    }

    #[test]
    fn endec_network_bandwidth_per_protocol_request() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;

        let network_bandwidth_per_protocol_request = NetworkBandwidthPerProtocolRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            get_test_filters(),
        );
        assert_eq!(network_bandwidth_per_protocol_request, NetworkBandwidthPerProtocolRequestDTO::decode(&network_bandwidth_per_protocol_request.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;

        let network_bandwidth_per_protocol_request = NetworkBandwidthPerProtocolRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            get_test_filters(),
        );
        assert_eq!(network_bandwidth_per_protocol_request.get_type(), NetworkBandwidthPerProtocolRequestDTO::get_data_type());
        assert_eq!(network_bandwidth_per_protocol_request.get_type(), super::DATA_TYPE);
    }
}