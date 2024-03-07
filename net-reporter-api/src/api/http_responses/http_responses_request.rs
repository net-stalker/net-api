use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;
use ion_rs::ReaderBuilder;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;

use super::http_responses_filters::HttpResponsesFiltersDTO;


const DATA_TYPE: &str = "http_responses_request";

#[derive(Debug, PartialEq, Eq)]
pub struct HttpClientsRequestDTO {
    start_date_time: i64,
    end_date_time: i64,
    filters: HttpResponsesFiltersDTO,
}
impl API for HttpClientsRequestDTO { }

impl HttpClientsRequestDTO {
    pub fn new(start_date_time: i64, end_date_time: i64, filters: HttpResponsesFiltersDTO) -> Self {
        HttpClientsRequestDTO {
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

    pub fn get_filters(&self) -> &HttpResponsesFiltersDTO {
        &self.filters
    }
}

impl Encoder for HttpClientsRequestDTO {
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

impl Decoder for HttpClientsRequestDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let start_date_time = binary_user_reader.read_i64().unwrap();
        
        binary_user_reader.next().unwrap();
        let end_date_time = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        
        let data = binary_user_reader.read_blob().unwrap();
        let filters = HttpResponsesFiltersDTO::decode(data.as_slice());
        
        HttpClientsRequestDTO::new(
            start_date_time,
            end_date_time,
            filters,
        )
    }
}

impl Typed for HttpClientsRequestDTO {
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

    use crate::api::http_responses::http_responses_request::HttpClientsRequestDTO;
    use crate::api::http_responses::http_responses_filters::HttpResponsesFiltersDTO;

    fn get_test_filters() -> HttpResponsesFiltersDTO {
        let http_responses = vec![200, 400];
        const INCLUDE_RESPONSE_MODE: bool = false;
        let endpoints = vec!["0.0.0.0".to_string(), "1.1.1.1".to_string()];
        const INCLUDE_ENDPOINTS_MODE: bool = true;
        
        HttpResponsesFiltersDTO::new(
            &http_responses,
            Some(INCLUDE_RESPONSE_MODE),
            &endpoints,
            Some(INCLUDE_ENDPOINTS_MODE),
            None,
            None,
        )
    }

    #[test]
    fn reader_correctly_read_encoded_http_responses_request_some_filters() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;

        let network_bandwidth_request = HttpClientsRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            get_test_filters(),
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
        assert_eq!(get_test_filters(), HttpResponsesFiltersDTO::decode(binary_user_reader.read_blob().unwrap().as_slice()));
        
        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_http_responses_request() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;

        let network_bandwidth_request = HttpClientsRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            get_test_filters(),
        );
        assert_eq!(network_bandwidth_request, HttpClientsRequestDTO::decode(&network_bandwidth_request.encode()));
    }
}