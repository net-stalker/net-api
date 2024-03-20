use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;
use ion_rs::ReaderBuilder;

use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

use super::http_request_methods_distribution_filters::HttpRequestMethodsDisributionFiltersDTO;


const DATA_TYPE: &str = "http_request_methods_disribution_request";

#[derive(Debug, PartialEq, Eq)]
pub struct HttpRequestMethodsDistributionRequestDTO {
    start_date_time: i64,
    end_date_time: i64,
    filters: HttpRequestMethodsDisributionFiltersDTO,
}
impl API for HttpRequestMethodsDistributionRequestDTO { }

impl HttpRequestMethodsDistributionRequestDTO {
    pub fn new(start_date_time: i64, end_date_time: i64, filters: HttpRequestMethodsDisributionFiltersDTO) -> Self {
        HttpRequestMethodsDistributionRequestDTO {
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

    pub fn get_filters(&self) -> &HttpRequestMethodsDisributionFiltersDTO {
        &self.filters
    }
}

impl Encoder for HttpRequestMethodsDistributionRequestDTO {
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
        writer.write_blob(self.filters.encode().as_slice()).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for HttpRequestMethodsDistributionRequestDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let start_date_time = binary_user_reader.read_i64().unwrap();
        
        binary_user_reader.next().unwrap();
        let end_date_time = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        let filters = HttpRequestMethodsDisributionFiltersDTO::decode(binary_user_reader.read_blob().unwrap().as_slice());

        HttpRequestMethodsDistributionRequestDTO::new(
            start_date_time,
            end_date_time,
            filters
        )
    }
}

impl Typed for HttpRequestMethodsDistributionRequestDTO {
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

    use crate::api::http_request_methods_distribution::http_request_methods_distribution_request::HttpRequestMethodsDistributionRequestDTO;
    use crate::api::http_request_methods_distribution::http_request_methods_distribution_filters::HttpRequestMethodsDisributionFiltersDTO;

    fn get_test_filters() -> HttpRequestMethodsDisributionFiltersDTO {
        let endpoints = vec!["0.0.0.0".to_string(), "1.1.1.1".to_string()];
        const INCLUDE_ENDPOINTS_MODE: bool = true;
        
        HttpRequestMethodsDisributionFiltersDTO::new(
            &endpoints,
            Some(INCLUDE_ENDPOINTS_MODE),
            None,
            None,
        )
    }

    #[test]
    fn reader_correctly_read_encoded_hrmd_request() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;

        let network_graph_request = HttpRequestMethodsDistributionRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            get_test_filters(),
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

        assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
        assert_eq!("filters", binary_user_reader.field_name().unwrap());
        assert_eq!(get_test_filters(), HttpRequestMethodsDisributionFiltersDTO::decode(binary_user_reader.read_blob().unwrap().as_slice()));
    }

    #[test]
    fn endec_hrmd_request() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;

        let network_graph_request = HttpRequestMethodsDistributionRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            get_test_filters(),
        );
        assert_eq!(network_graph_request, HttpRequestMethodsDistributionRequestDTO::decode(&network_graph_request.encode()));
    }
}