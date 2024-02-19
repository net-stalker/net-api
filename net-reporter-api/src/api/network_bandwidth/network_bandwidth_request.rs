use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;
use ion_rs::ReaderBuilder;

use ion_rs::StreamItem;
use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;

use super::network_bandwidth_filters::NetworkBandwidthFiltersDTO;


const DATA_TYPE: &str = "network_bandwidth_request";

#[derive(Debug, PartialEq, Eq)]
pub struct NetworkBandwidthRequestDTO {
    start_date_time: i64,
    end_date_time: i64,
    filters: Option<NetworkBandwidthFiltersDTO>,
}
impl API for NetworkBandwidthRequestDTO { }

impl NetworkBandwidthRequestDTO {
    pub fn new (start_date_time: i64, end_date_time: i64, filters: Option<&NetworkBandwidthFiltersDTO>) -> Self {
        NetworkBandwidthRequestDTO {
            start_date_time,
            end_date_time,
            filters: filters.map(|filters| filters.into()),
        }
    }

    pub fn get_start_date_time (&self) -> i64 {
        self.start_date_time
    }

    pub fn get_end_date_time (&self) -> i64 {
        self.end_date_time
    }

    pub fn get_filters(&self) -> Option<&NetworkBandwidthFiltersDTO> {
        self.filters.as_ref()
    }
}

impl Encoder for NetworkBandwidthRequestDTO {
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
        match self.filters.as_ref() {
            Some(filters) => {
                let data = filters.encode();
                writer.write_blob(data.as_slice()).unwrap();
            },
            None => writer.write_null(IonType::Blob).unwrap(),
        }

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkBandwidthRequestDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let start_date_time = binary_user_reader.read_i64().unwrap();
        
        binary_user_reader.next().unwrap();
        let end_date_time = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        let filters = match binary_user_reader.current() {
            StreamItem::Value(_) => {
                let data = binary_user_reader.read_blob().unwrap();
                Some(NetworkBandwidthFiltersDTO::decode(data.as_slice()))
            },
            _ => {
                None
            }
        };

        NetworkBandwidthRequestDTO::new(
            start_date_time,
            end_date_time,
            filters.as_ref(),
        )
    }
}

impl Typed for NetworkBandwidthRequestDTO {
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

    use crate::api::network_bandwidth::network_bandwidth_filters::NetworkBandwidthFiltersDTO;
    use crate::api::network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO;

    fn get_test_filters() -> NetworkBandwidthFiltersDTO {
        NetworkBandwidthFiltersDTO::new(
            &["TCP".to_string(), "UDP".to_string()],
            true,
            &["0.0.0.0".to_string(), "1.1.1.1".to_string()],
            false,
        )
    }

    #[test]
    fn reader_correctly_read_encoded_nb_request_some_filters() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;

        let network_bandwidth_request = NetworkBandwidthRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            Some(&get_test_filters())
        );
        
        let mut binary_user_reader = ReaderBuilder::new().build(network_bandwidth_request.encode()).unwrap();

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
        assert_eq!(get_test_filters(), NetworkBandwidthFiltersDTO::decode(binary_user_reader.read_blob().unwrap().as_slice()));
        
        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn reader_correctly_read_encoded_nb_request_none_filters() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;

        let network_bandwidth_request = NetworkBandwidthRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            None,
        );
        
        let mut binary_user_reader = ReaderBuilder::new().build(network_bandwidth_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("start_date_time", binary_user_reader.field_name().unwrap());
        assert_eq!(START_DATE_TIME, binary_user_reader.read_i64().unwrap());
        
        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("end_date_time", binary_user_reader.field_name().unwrap());
        assert_eq!(END_DATE_TIME,  binary_user_reader.read_i64().unwrap());

        assert_eq!(StreamItem::Null(IonType::Blob), binary_user_reader.next().unwrap());
        assert_eq!("filters", binary_user_reader.field_name().unwrap());
        
        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_ng_request() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;

        let network_bandwidth_request = NetworkBandwidthRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            None,
        );
        assert_eq!(network_bandwidth_request, NetworkBandwidthRequestDTO::decode(&network_bandwidth_request.encode()));
    }
}