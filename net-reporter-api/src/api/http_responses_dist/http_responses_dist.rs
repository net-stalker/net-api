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

use super::http_responses_bucket::HttpResponsesBucketDTO;


const DATA_TYPE: &str = "http_responses_dist";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpResponsesDistDTO {
    http_responses_buckets: Vec<HttpResponsesBucketDTO>,
}
impl API for HttpResponsesDistDTO { }

impl HttpResponsesDistDTO {
    pub fn new(http_responses_buckets: &[HttpResponsesBucketDTO]) -> Self {
        HttpResponsesDistDTO { http_responses_buckets: http_responses_buckets.to_vec() }
    }

    pub fn get_http_responses_buckets(&self) -> &[HttpResponsesBucketDTO] {
        &self.http_responses_buckets
    }
}

impl Encoder for HttpResponsesDistDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("http_responses_buckets");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        self.http_responses_buckets.iter().for_each(|http_responses_bucket| {
            let data = http_responses_bucket.encode();
            writer.write_blob(data.as_slice()).unwrap();
        });
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for HttpResponsesDistDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let http_responses_buckets_elements = binary_user_reader.read_all_elements().unwrap();
        let mut http_responses_buckets = Vec::with_capacity(http_responses_buckets_elements.len());
        http_responses_buckets_elements.iter().for_each(|element| {
            let data = element.as_blob().unwrap();
            let http_responses_bucket = HttpResponsesBucketDTO::decode(data);
            http_responses_buckets.push(http_responses_bucket);
        });

        Self::new(&http_responses_buckets)
    }
}

impl Typed for HttpResponsesDistDTO {
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

    use crate::api::http_responses_dist::http_responses_dist::HttpResponsesDistDTO;
    use crate::api::http_responses_dist::http_responses_bucket::HttpResponsesBucketDTO;


    #[test]
    fn reader_correctly_read_encoded_http_responses_dist() {
        const BUCKET_TIMESTAMP: i64 = i64::MAX;
        const RESPONSE_CODE: i64 = i64::MIN;
        const AMOUNT: i64 = 0;

        let http_response_bucket = HttpResponsesBucketDTO::new(
            BUCKET_TIMESTAMP,
            RESPONSE_CODE,
            AMOUNT,
        );

        let http_responses_buckets = vec![http_response_bucket];

        let http_responses_dist = HttpResponsesDistDTO::new(
            &http_responses_buckets,
        );

        let mut binary_user_reader = ReaderBuilder::new().build(http_responses_dist.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("http_responses_buckets", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        assert_eq!(elements.len(), http_responses_buckets.len());
        for (element, http_response_bucket_core) in elements.iter().zip(http_responses_buckets.as_slice()) {
            let encoded_http_responses_bucket = HttpResponsesBucketDTO::decode(element.as_blob().unwrap());
            assert_eq!(encoded_http_responses_bucket, *http_response_bucket_core);
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_http_responses_dist() {
        const BUCKET_TIMESTAMP: i64 = i64::MAX;
        const RESPONSE_CODE: i64 = i64::MIN;
        const AMOUNT: i64 = 0;

        let http_response_bucket = HttpResponsesBucketDTO::new(
            BUCKET_TIMESTAMP,
            RESPONSE_CODE,
            AMOUNT,
        );

        let http_responses_buckets = vec![http_response_bucket];

        let http_responses_dist = HttpResponsesDistDTO::new(
            &http_responses_buckets,
        );

        assert_eq!(http_responses_dist, HttpResponsesDistDTO::decode(&http_responses_dist.encode()));
    }
}