use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;
use ion_rs::element::reader::ElementReader;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;

use super::http_requests_bucket::HttpRequestsBucketDTO;


const DATA_TYPE: &str = "total_http_requests";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TotalHttpRequestsDTO {
    http_requests_buckets: Vec<HttpRequestsBucketDTO>,
}
impl API for TotalHttpRequestsDTO { }

impl TotalHttpRequestsDTO {
    pub fn new(http_requests_buckets: &[HttpRequestsBucketDTO],) -> Self {
        TotalHttpRequestsDTO {
            http_requests_buckets: http_requests_buckets.to_vec(),
        }
    }

    pub fn get_http_requests_buckets(&self) -> &[HttpRequestsBucketDTO] {
        &self.http_requests_buckets
    }
}

impl Encoder for TotalHttpRequestsDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("http_requests_buckets");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        self.http_requests_buckets.iter().for_each(|http_requests_bucket| {
            let data = http_requests_bucket.encode();
            writer.write_blob(data.as_slice()).unwrap();
        });
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for TotalHttpRequestsDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let total_http_requests_elements = binary_user_reader.read_all_elements().unwrap();
        let mut http_requests_buckets = Vec::with_capacity(total_http_requests_elements.len());
        total_http_requests_elements.iter().for_each(|element| {
            let data = element.as_blob().unwrap();
            let bucket = HttpRequestsBucketDTO::decode(data);
            http_requests_buckets.push(bucket);
        });

        Self::new(&http_requests_buckets)
    }
}

impl Typed for TotalHttpRequestsDTO {
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
    use ion_rs::element::reader::ElementReader;

    use net_core_api::encoder_api::Encoder;
    use net_core_api::decoder_api::Decoder;

    use crate::api::total_http_requests::http_requests_bucket::HttpRequestsBucketDTO;
    use crate::api::total_http_requests::total_http_requests::TotalHttpRequestsDTO;


    #[test]
    fn reader_correctly_read_total_http_requests() {
        const FIRST_BUCKET_TIMESTAMP: i64 = i64::MAX;
        const FIRST_TOTAL_REQUESTS: i64 = i64::MAX;

        let first_bandwidth_bucket = HttpRequestsBucketDTO::new(
            FIRST_BUCKET_TIMESTAMP,
            FIRST_TOTAL_REQUESTS
        );

        const SECOND_BUCKET_TIMESTAMP: i64 = i64::MAX;
        const SECOND_TOTAL_REQUESTS: i64 = i64::MAX;

        let second_bandwidth_bucket = HttpRequestsBucketDTO::new(
            SECOND_BUCKET_TIMESTAMP,
            SECOND_TOTAL_REQUESTS
        );

        let bandwidth_buckets = vec![
            first_bandwidth_bucket,
            second_bandwidth_bucket
        ];

        let network_bandwidth = TotalHttpRequestsDTO::new(
            &bandwidth_buckets
        );

        let mut binary_user_reader = ReaderBuilder::new().build(network_bandwidth.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("http_requests_buckets", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        assert_eq!(elements.len(), bandwidth_buckets.len());
        for (element, bandwidth_bucket_core) in elements.iter().zip(bandwidth_buckets.as_slice()) {
            let encoded_endpoint = HttpRequestsBucketDTO::decode(element.as_blob().unwrap());
            assert_eq!(encoded_endpoint, *bandwidth_bucket_core);
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_total_http_requests() {
        const FIRST_BUCKET_TIMESTAMP: i64 = i64::MAX;
        const FIRST_TOTAL_REQUESTS: i64 = i64::MAX;

        let first_bandwidth_bucket = HttpRequestsBucketDTO::new(
            FIRST_BUCKET_TIMESTAMP,
            FIRST_TOTAL_REQUESTS
        );

        const SECOND_BUCKET_TIMESTAMP: i64 = i64::MAX;
        const SECOND_TOTAL_REQUESTS: i64 = i64::MAX;

        let second_bandwidth_bucket = HttpRequestsBucketDTO::new(
            SECOND_BUCKET_TIMESTAMP,
            SECOND_TOTAL_REQUESTS
        );

        let bandwidth_buckets = vec![
            first_bandwidth_bucket,
            second_bandwidth_bucket
        ];

        let network_bandwidth = TotalHttpRequestsDTO::new(
            &bandwidth_buckets
        );


        assert_eq!(network_bandwidth, TotalHttpRequestsDTO::decode(&network_bandwidth.encode()));
    }
}